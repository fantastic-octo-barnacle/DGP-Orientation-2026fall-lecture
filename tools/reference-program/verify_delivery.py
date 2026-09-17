"""内部跨进程验证。使用已构建产物；不随候选人材料发布。"""
import argparse
from contextlib import contextmanager
import json
import os
from pathlib import Path
import queue
import socket
import subprocess
import threading


@contextmanager
def running(command, cwd=None):
    process = subprocess.Popen(
        command, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        text=True, encoding="utf-8",
        env={**os.environ, "PYTHONUTF8": "1"},
        creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0,
    )
    lines = queue.Queue()
    def read():
        for line in process.stdout:
            lines.put(line)
    worker = threading.Thread(target=read, daemon=True)
    worker.start()
    try:
        line = lines.get(timeout=20)
        assert line.startswith("LISTENING "), line
        port = int(line.strip().rsplit(":", 1)[1])
        yield port
    finally:
        process.terminate()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=5)
        worker.join(timeout=1)
        process.stdout.close()
        process.stderr.close()


def run(command, cwd=None, input=None):
    result = subprocess.run(command, cwd=cwd, input=input, text=True, encoding="utf-8",
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=30,
        env={**os.environ, "PYTHONUTF8": "1"},
        creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0)
    assert result.returncode == 0, (command, result.stdout, result.stderr)
    return result.stdout


def raw(port, requests):
    output = []
    with socket.create_connection(("127.0.0.1", port), timeout=3) as connection:
        with connection.makefile("rb") as reader:
            for request in requests:
                connection.sendall(request + b"\n")
                output.append(json.loads(reader.readline()))
    return output


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidate-root", type=Path, required=True)
    parser.add_argument("--reference", type=Path, required=True)
    args = parser.parse_args()
    root = args.candidate_root.resolve()
    reference = str(args.reference.resolve())
    pyroot = root / "projects/python"
    python = pyroot / ".venv" / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
    suffix = ".exe" if os.name == "nt" else ""
    rustroot = root / "projects/rust"
    rustclient = rustroot / "client-sync/target/debug" / ("rm-client-sync" + suffix)

    with running([reference, "server", "--bind", "127.0.0.1:0"]) as port:
        output = run([str(python), "-m", "tool_service.client", "--port", str(port)],
            input="1\n2\nhello\nq\n")
        assert "pong" in output and "hello" in output
        for stage in ("baseline", "text", "numbers", "store"):
            run([str(python), "self-check/check.py", stage, "--port", str(port)], cwd=pyroot)
        assert json.loads(run([str(rustclient), f"127.0.0.1:{port}"]))["data"] == "pong"

    with running([str(python), "-m", "tool_service.server", "--port", "0"]) as port:
        requests = [
            {"id": 1, "action": "ping"},
            {"id": 2, "action": "echo", "data": "你好"},
            {"id": 3, "action": "text_stats", "text": "a\n🙂"},
        ]
        output = run([reference, "client", "--address", f"127.0.0.1:{port}"],
            input="".join(json.dumps(v, ensure_ascii=False) + "\n" for v in requests))
        actual = [json.loads(line) for line in output.splitlines()]
        assert [v["data"] for v in actual] == ["pong", "你好", {"characters": 3, "lines": 2}]
        for stage in ("baseline", "text"):
            run([str(python), "self-check/check.py", stage, "--port", str(port)], cwd=pyroot)
        errors = raw(port, [b"{", b'{"id":true,"action":"ping"}',
            b'{"id":1,"action":"echo","data":"\\ud800"}',
            b'{"id":2,"action":"ping"}'])
        assert [v.get("error", {}).get("code") for v in errors[:3]] == [
            "invalid_json", "invalid_request", "invalid_json"]
        assert errors[-1]["data"] == "pong"

    for directory, binary in (("server-sync", "rm-server-sync"), ("server-async", "rm-server-async")):
        path = rustroot / directory / "target/debug" / (binary + suffix)
        with running([str(path), "127.0.0.1:0"]) as port:
            result = run([reference, "client", "--address", f"127.0.0.1:{port}"],
                input='{"id":3,"action":"ping"}\n')
            assert json.loads(result)["data"] == "pong"
            assert json.loads(run([str(rustclient), f"127.0.0.1:{port}"]))["data"] == "pong"
    print("PASS: Python/reference bidirectional interoperability, all reference actions, both Rust baselines")


if __name__ == "__main__":
    main()
