---
theme: "@ktym4a/slidev-theme-ktym4a"
title: 计算机开发基础
info: RM 软件组招新基础课
transition: fade
themeConfig:
  baseColor: red
  colorPattern: rotation
addons:
  - slidev-addon-counter
layout: cover
class: cover-slide
---

<style>
@import './styles/index.css';
</style>

# 计算机开发基础

<div class="cover-meta">RM 软件组招新培训</div>

---

# TOC

<div class="course-toc" role="list" aria-label="课程目录">
  <div class="course-toc-item text-blue" role="listitem"><span class="course-toc-number">01</span><strong>操作系统与 WSL</strong></div>
  <div class="course-toc-item text-green" role="listitem"><span class="course-toc-number">05</span><strong>环境与依赖管理</strong></div>
  <div class="course-toc-item text-blue" role="listitem"><span class="course-toc-number">02</span><strong>终端、Shell 与命令行</strong></div>
  <div class="course-toc-item text-green" role="listitem"><span class="course-toc-number">06</span><strong>开发规范与代码质量管理</strong></div>
  <div class="course-toc-item text-blue" role="listitem"><span class="course-toc-number">03</span><strong>文件与文件系统</strong></div>
  <div class="course-toc-item text-green" role="listitem"><span class="course-toc-number">07</span><strong>Git 与版本控制</strong></div>
  <div class="course-toc-item text-blue" role="listitem"><span class="course-toc-number">04</span><strong>环境变量，PATH</strong></div>
  <div class="course-toc-item text-green" role="listitem"><span class="course-toc-number">08</span><strong>编辑器集成</strong></div>
</div>

<!--
开场与目录 3 分钟｜累计 00:00–00:03
净讲课共 120 分钟（含演示）；休息、互动、答疑另计。
-->

---
layout: section
---

# <Counter :level="1" /> 操作系统与 WSL

<p>程序工作的地基</p>

<!--
本章 8 分钟｜累计 00:03–00:11
-->

---

# <Counter /> 操作系统负责管理公共资源

<div class="two-col">
  <div class="stack">
    <div class="diagram-box text-pink">程序与进程</div>
    <div class="diagram-box text-mauve">文件系统</div>
    <div class="diagram-box text-red">内存与设备</div>
  </div>
  <div class="stack">
    <div class="diagram-box text-peach">用户与权限</div>
    <div class="diagram-box text-green">网络与通信</div>
    <div class="diagram-box text-sky">启动、调度与隔离</div>
  </div>
</div>

<p class="lead">操作系统让程序有地方运行、有资源可用。</p>

---

# <Counter /> 操作系统的谱系

<div class="os-tree" role="img" aria-label="操作系统与 UNIX、Unix-like 以及 Windows NT 家族的简化关系图">
  <div class="os-tree-main">
    <div class="os-tree-column text-pink">
      <div class="os-tree-node os-tree-category">
        <strong>Unix-like</strong>
        <span>类 Unix 系统</span>
      </div>
      <div class="os-tree-children os-tree-children--three">
        <div class="os-tree-column text-red">
          <div class="os-tree-node os-tree-family">
            <strong>UNIX</strong>
            <span>标准 / 认证语境</span>
          </div>
          <div class="os-tree-children os-tree-children--three os-tree-children--compact">
            <div class="os-tree-column text-red">
              <div class="os-tree-node os-tree-leaf"><strong>macOS</strong></div>
            </div>
            <div class="os-tree-column text-red">
              <div class="os-tree-node os-tree-leaf"><strong>AIX</strong></div>
            </div>
            <div class="os-tree-column text-red">
              <div class="os-tree-node os-tree-leaf"><strong>Solaris</strong></div>
            </div>
          </div>
        </div>
        <div class="os-tree-column text-green">
          <div class="os-tree-node os-tree-family">
            <strong>BSD 家族</strong>
            <span>Unix-like 家族</span>
          </div>
          <div class="os-tree-children os-tree-children--three os-tree-children--compact">
            <div class="os-tree-column text-green">
              <div class="os-tree-node os-tree-leaf"><strong>FreeBSD</strong></div>
            </div>
            <div class="os-tree-column text-green">
              <div class="os-tree-node os-tree-leaf"><strong>OpenBSD</strong></div>
            </div>
            <div class="os-tree-column text-green">
              <div class="os-tree-node os-tree-leaf"><strong>NetBSD</strong></div>
            </div>
          </div>
        </div>
        <div class="os-tree-column text-sky">
          <div class="os-tree-node os-tree-family">
            <strong>Linux 生态</strong>
            <span>内核与发行版</span>
          </div>
          <div class="os-tree-children os-tree-children--two os-tree-children--compact">
            <div class="os-tree-column text-sky">
              <div class="os-tree-node os-tree-leaf"><strong>Linux 内核</strong></div>
            </div>
            <div class="os-tree-column text-sky">
              <div class="os-tree-node os-tree-leaf os-tree-leaf--distribution">
                <strong>Linux 发行版</strong>
                <span>Ubuntu · Fedora · Arch Linux</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="os-tree-column text-mauve">
      <div class="os-tree-node os-tree-category">
        <strong>其他操作系统家族</strong>
      </div>
      <div class="os-tree-children os-tree-children--one os-tree-children--compact">
        <div class="os-tree-column text-mauve">
          <div class="os-tree-node os-tree-family">
            <strong>Windows NT</strong>
            <span>操作系统家族</span>
          </div>
          <div class="os-tree-children os-tree-children--three os-tree-children--compact">
            <div class="os-tree-column text-mauve">
              <div class="os-tree-node os-tree-leaf"><strong>Windows 10</strong></div>
            </div>
            <div class="os-tree-column text-mauve">
              <div class="os-tree-node os-tree-leaf"><strong>Windows 11</strong></div>
            </div>
            <div class="os-tree-column text-mauve">
              <div class="os-tree-node os-tree-leaf"><strong>Windows Server</strong></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<div class="os-tree-notes">
  <p><code>UNIX</code> 指通过了国际开放标准认证组织（The Open Group）单一 UNIX 规范认证（Single UNIX Specification）的系统；</p>
  <p><code>Unix-like</code> 是描述性术语。</p>
  <p class="source-note">术语参考：The Open Group · UNIX / Single UNIX Specification</p>
  <p class="source-note">本图为教学用简化关系图。</p>
</div>

---

# <Counter /> WSL：在 Windows 中使用 Linux

<div class="flow">
  <div class="diagram-box text-blue">Windows<br>日常桌面环境</div>
  <div class="flow-arrow">↔</div>
  <div class="diagram-box text-peach">WSL<br>Linux 运行环境</div>
</div>

<p class="lead"><em>Windows Subsystem for Linux (WSL)</em><br>是 Windows 提供的工具，让你在 Windows 中方便地使用 Linux 环境。</p>

<div class="activity-box text-green mt-5">
  <h3>怎么理解它？</h3>
  <p>相比传统虚拟机，WSL 不需要单独安装、启动和维护一套完整系统，和 Windows 之间的交互也更方便。</p>
</div>

<p class="source-note">本课主要以 WSL2 为例；WSL 提供了诸多 Linux 发行版，比如 Ubuntu、Debian、openSUSE 等。</p>

<!--
【演示】搜索 WSL 文档
注意：微软官方文档中文翻译很烂
-->

---

## WSL 的便利性

<div class="card-grid three">
  <div class="card text-pink">
    <h3>文件系统</h3>
    <p>Windows 和 WSL 可以访问彼此的一部分文件，例如在 WSL 中访问 Windows 的 C 盘文件。</p>
  </div>
  <div class="card text-peach">
    <h3>网络通信</h3>
    <p>WSL 中运行的开发服务，Windows 可以直接访问。</p>
  </div>
  <div class="card text-sky">
    <h3>VS Code</h3>
    <p>可以用 Windows 中的 VS Code 编辑代码，同时让代码和工具运行在 WSL 的 Linux 环境中。</p>
  </div>
</div>

<div class="callout text-mauve mt-5">
  <h3>但它们不是同一个环境</h3>
  <p>Windows 和 WSL 可以协作，但两边安装的软件、PATH 和环境仍可能不同。</p>
</div>

<!--
过渡：操作系统提供能力，终端是其中一种文本入口。
-->

---
layout: section
---

# <Counter :level="1" /> 终端、Shell 与命令行

<p>如何运行一个程序</p>

<!--
本章 24 分钟｜累计 00:11–00:35（含演示）
-->

---
layout: two-cols-header
---

# <Counter /> 终端是窗口，Shell 是程序

::left::

<br>
<div>
  <div class="architecture-node architecture-main-node text-red">
    <strong>用户</strong>
    <span>输入文字，观察结果</span>
  </div>
  <div class="architecture-arrow">↓ 输入 / 观察</div>
  <div class="architecture-node architecture-main-node text-pink">
    <strong>终端模拟器</strong>
    <span>显示文本、接收键盘输入</span>
  </div>
  <div class="architecture-arrow">↓ 启动 Shell、传递文本</div>
  <div class="architecture-node architecture-main-node text-peach">
    <strong>Shell</strong>
    <span>读取并解释命令</span>
  </div>
  <div class="architecture-arrow">↓ 请求操作系统服务</div>
  <div class="architecture-node architecture-main-node architecture-kernel text-mauve">
    <strong>操作系统内核</strong>
    <span>管理进程、文件、内存和硬件设备</span>
  </div>
</div>

::right::

<br>
<div class="architecture-side-note text-pink">
  <strong>终端模拟器</strong>
  <p>负责显示文本、接收键盘输入的程序。它会将用户的输入传递给 Shell，并将 Shell 的输出显示给用户。</p>
  <p class="small muted mt-2">例：Windows Terminal、iTerm2、GNOME Terminal、...</p>
</div>
<br>
<div class="architecture-side-note text-peach">
  <strong>Shell</strong>
  <p>在终端中运行，负责读取并解释命令的程序。它会通过操作系统提供的接口请求内核服务：启动应用或其他程序、读写文件，以及进行其他系统交互。</p>
  <p class="small muted mt-2">例：PowerShell、pwsh、Bash、Zsh、fish、...</p>
</div>

<!--
【演示】Win Terminal 同时开多个 shell
CMD 和 PowerShell 是不同的 shell




`powershell.exe`  → Windows PowerShell 5.1，基于 **.NET Framework**
`pwsh.exe`        → PowerShell 7+，基于 **.NET（Core）** 的跨平台版本

* 有一定的兼容性和行为变化。
-->

---

# <Counter /> Shell 会维护自己的状态

<div class="two-col">
  <div class="diagram-box text-red">
    <h3>当前目录</h3>
    <p>决定相对路径从哪里开始解释。</p>
  </div>
  <div class="diagram-box text-peach">
    <h3>环境变量</h3>
    <p>启动其他程序时会传递的一组名称和值。</p>
  </div>
</div>

<div class="flow mt-4">
  <div class="diagram-box text-pink"><code>cd path/to/your/project</code></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-mauve">Shell 更新自己的当前目录</div>
</div>

<div class="callout text-sky mt-4">
  <p><strong>内建命令：</strong><code>cd</code> 等内建命令由 Shell 自己处理，会改变 Shell 自身的状态。</p>
</div>

<p class="small muted">目录（文件系统）和环境变量的细节，会在后面单独解释。</p>

---

# <Counter /> Shell 的使用

<div class="flow">
  <div class="diagram-box text-mauve">
    Shell&nbsp;&nbsp;&nbsp;<em>解释命令</em>
  </div>
  <div class="flow-arrow">⇒ 启动 ⇒</div>
  <div class="diagram-box text-green">
    外部程序&nbsp;&nbsp;&nbsp;<em>Git · Python · 自编译程序</em>
  </div>
</div>

<div class="two-col mt-5">
  <div class="card text-green">
    <h3>程序完成具体工作</h3>
    <p>例如读取文件、运行代码，或与其他程序通信。</p>
  </div>
  <div class="card text-peach">
    <h3>结果回到终端</h3>
    <p>程序输出文本，终端模拟器负责把它显示出来。</p>
  </div>
</div>

<p class="lead">命令有时是 Shell 的内建功能，有时是 Shell 要启动的程序。</p>

---

## 使用 Shell 运行命令

<div class="terminal-grid mt-4">
  <div>
    <div class="terminal-label text-blue">Windows · PowerShell</div>

```text
PS> Write-Output "hello"
hello

PS> Get-Date
<日期和时间>

PS> Get-Location
<当前目录>
```

  </div>
  <div>
    <div class="terminal-label text-peach">macOS / Linux · Bash</div>

```text
$ echo hello
hello

$ date
<日期和时间>

$ pwd
<当前目录>
```

  </div>
</div>

<p class="terminal-legend"><code>PS&gt;</code> / <code>$</code> 是 Shell 给出的提示符（prompt）；下面的输出会因机器而异。</p>

---

## 使用 Shell 运行程序

<div class="program-example-grid mt-3">
  <div class="program-example-column">
<div class="terminal-label text-green">Windows · PowerShell · 命令行程序</div>

```text
PS> git --version
git version 2.x

PS> python -c "print('hello from Python')"
hello from Python

PS> curl.exe --version
curl 8.x
```

<div class="terminal-label text-peach program-example-label">Windows · PowerShell · GUI 程序</div>

```text
PS> notepad
PS> calc
PS> mspaint
```

  </div>
  <div class="program-example-column">
<div class="terminal-label text-green">macOS / Linux · Bash · 命令行程序</div>

```text
$ git --version
git version 2.x

$ python3 -c 'print("hello from Python")'
hello from Python

$ curl --version
curl 8.x
```

<div class="terminal-label text-peach program-example-label">macOS · Bash · GUI 程序</div>

```text
$ open -a TextEdit
$ open -a Calculator
$ open -a Preview
```

  </div>
</div>

---

# <Counter /> Shell 如何解释一行命令

## 命令名与参数

```bash
python3 -c "print('hello from Python')"
```

<table class="compact">
  <thead><tr><th>命令名：调用谁（程序）</th><th>参数 1</th><th>参数 2：整段代码</th></tr></thead>
  <tbody><tr><td>Python 3.x</td><td><code>-c</code></td><td><code>print('hello from Python')</code></td></tr></tbody>
</table>

<div class="flow mt-5">
  <div class="diagram-box text-peach">Shell 处理命令行：<br>识别命令和目标程序、分割参数</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">Python 解释参数：<br>执行传入的代码</div>
</div>

<p class="lead">Shell 负责传入参数，命令决定参数的含义。</p>
<p class="small muted">命令也可以是 Shell 的内建功能，不一定是外部程序。</p>

<!--
用时约 1 分钟。从前面的 Python 运行示例接过来，不引入新命令。
Shell 传入两个参数：-c 和整段代码。外层双引号用于分组，不是参数内容；内部单引号保留，交给 Python 解释。
Python 将 -c 后面的参数作为代码执行，输出 hello from Python。下一页再展开空格与引号的规则。
本机若使用 python3 命令，替换命令名即可。
只建立简单命令的模型，不将其概括成所有 Shell 语法；暂不讲展开、管道和重定向。
-->

---

## 空格分隔，引号分组

<div class="program-example-grid mt-4">
  <div>
    <div class="terminal-label text-peach">Bash · 两个参数</div>

```bash
echo hello RM
echo hello    RM
```

```text
hello RM
hello RM
```

<p class="small">参数：<code>hello</code>、<code>RM</code>；<code>hello</code>、<code>RM</code></p>
  </div>
  <div>
    <div class="terminal-label text-green">Bash · 一段含空格的文字</div>

```bash
echo "hello RM"
echo "hello    RM"
```

```text
hello RM
hello    RM
```

<p class="small">参数：<code>hello RM</code>；<code>hello&nbsp;&nbsp;&nbsp;RM</code></p>
  </div>
</div>

- 未被引用的空格或 Tab 通常分隔各部分；连续多个空格不是空参数。
- 引号可以让含空格的内容留在一个参数中；这里的外层引号不传给命令。
- 使用成对的英文半角引号；单引号与双引号并不总是等价。

<p class="small muted">引号未闭合时，Shell 可能继续等待输入；可用 <kbd>Ctrl</kbd>+<kbd>C</kbd> 取消，再重新输入。</p>

<!--
用时约 2 分钟。echo 将文字输出到终端，不引入格式字符串。
指着参数标注比较两个参数与一个参数；输出相同不代表拆分相同，不能只凭 echo 的输出判断参数数量。
这里只分析没有展开或特殊操作符的简单示例；单双引号的展开差异留待后续学习。
-->

---

## 文字本身带引号怎么办？

<div class="program-example-grid mt-4">
  <div>
    <div class="terminal-label text-green">Bash · 外层换一种引号</div>

```bash
echo 'hello "RM"'
echo "it's ready"
```

<p class="small">外层单引号保留里面的双引号；外层双引号也可以保留里面的单引号。</p>
  </div>
  <div>
    <div class="terminal-label text-peach">Bash · 在双引号内转义</div>

```bash
echo "hello \"RM\""
```

<p class="small"><code>\"</code> 让这个双引号成为文字，而不是结束外层引号。</p>
  </div>
</div>

<p><strong>转义：</strong>让原本有语法含义的字符，在这里作为普通文字。</p>
<p class="small">第一条与右侧命令都输出 <code>hello "RM"</code>，文字仍是一个参数。</p>
<p class="small">注意：Bash 的单引号内不能用 <code>\'</code> 保留单引号；例如 <code>it's ready</code>，可改用外层双引号。</p>
<p class="small muted">PowerShell 的转义符是反引号 <code>&#96;</code>，不是反斜杠；例如 <code>Write-Output "hello &#96;"RM&#96;""</code>。也可用外层单引号包住含双引号的文字。</p>

<!--
用时约 1 分钟，接在“引号分组”之后。先讲换外层引号，再讲 Bash 双引号中的反斜杠。
强调外层引号负责分组，里面需要保留的引号属于参数内容；不要教成“所有特殊字符前加反斜杠就行”。
Bash 单引号中反斜杠没有转义作用；单双引号仍有展开差异，本页只使用普通文字，不展开变量规则。
PowerShell 只作平台提醒，不延伸外部程序参数传递和多层嵌套。
-->

---

# <Counter /> 程序如何解释参数

<div class="terminal-label text-peach">Linux / macOS · 列出当前目录的内容</div>

```bash
ls -l -a .
```

<table class="compact">
  <thead><tr><th>部分</th><th>角色</th><th>含义</th></tr></thead>
  <tbody>
    <tr><td><code>-l</code>、<code>-a</code></td><td>短选项</td><td>详细列出；包含隐藏项</td></tr>
    <tr><td><code>.</code></td><td>位置参数</td><td>操作对象：当前目录</td></tr>
    <tr><td><code>--version</code></td><td>长选项</td><td>如 <code>git --version</code> 中的版本查询</td></tr>
    <tr><td><code>--color=auto</code></td><td>长选项及其值</td><td>GNU ls：让 <code>--color</code> 的值为 <code>auto</code></td></tr>
  </tbody>
</table>

<p class="lead">选项也是参数；有的选项是开关，有的还需要值。</p>
<p class="small muted">位置参数的含义由位置和命令规则决定。目录与路径下一章再展开。</p>

<!--
用时约 1.5 分钟。按命令名、两个选项、操作对象读一遍，不在此讲 ls 的完整用法。
GNU ls 的长选项不套用到 macOS 自带 ls；Windows PowerShell 的 ls 别名也不直接照搬这些参数。
-->

---

## 常见写法

<table class="compact mt-4">
  <thead><tr><th>形式</th><th>例子</th><th>注意</th></tr></thead>
  <tbody>
    <tr><td>合并短选项</td><td><code>ls -la .</code></td><td>这里等同于 <code>ls -l -a .</code>，不是处处适用</td></tr>
    <tr><td>选项与值分开</td><td><code>git -C . --version</code></td><td><code>.</code> 是 <code>-C</code> 的值，不是位置参数</td></tr>
    <tr><td>用 <code>=</code> 连接值</td><td><code>ls --color=auto .</code></td><td>GNU ls；不能随意改为空格</td></tr>
    <tr><td>子命令</td><td><code>git status --short</code></td><td><code>status</code> 选择 Git 的功能，后面是它的选项</td></tr>
  </tbody>
</table>

<p class="small muted">PowerShell 自身的命令常用 <code>-Name</code> 这样的参数名，不是多个短选项；调用 Git 等外部程序时，则要查该程序的参数规则。</p>

<!--
用时约 1.5 分钟。git -C . 表示让 Git 在当前目录运行，这里只示范选项吃掉后面的值。
GNU ls 的 --color 接受可选值，使用 = 的写法；不教“等号和空格总能互换”。
git status 只用于辨认子命令，不要求执行或提前教授仓库操作。
不要猜 -v 一定是版本（它也常表示 verbose），也不要凭选项长度猜含义。
-->

---

# <Counter /> 不同 Shell，同名命令未必相同

<table class="compact">
  <thead><tr><th>区别</th><th>Bash / Zsh</th><th>PowerShell</th></tr></thead>
  <tbody>
    <tr><td>常见命令风格</td><td><code>ls -a .</code></td><td><code>Get-ChildItem -Force .</code></td></tr>
    <tr><td>管道传递什么</td><td>字节流，常作为文本处理</td><td>cmdlet 之间通常传递对象，可保留属性</td></tr>
    <tr><td>转义字符</td><td>反斜杠 <code>\</code></td><td>反引号 <code>&#96;</code></td></tr>
  </tbody>
</table>

<p><strong>Windows 上的 PowerShell 提供了许多熟悉名字的别名（alias）：</strong></p>
<p class="small"><code>ls</code> → <code>Get-ChildItem</code>；<code>cat</code> → <code>Get-Content</code>；<code>cp</code> → <code>Copy-Item</code></p>
<p class="lead">别名只是另一个名字，不会把 Unix 参数翻译成 PowerShell 参数。</p>
<p class="small muted">不要直接照搬 <code>ls -la</code>；可用 <code>Get-Alias ls</code> 查看别名，用 <code>Get-Help Get-ChildItem</code> 查实际命令的用法。</p>

<!--
用时约 1.5 分钟。Bash 与 Zsh 同属 Unix 风格 Shell，但语法并非完全相同；这里合并比较常见使用习惯。
PowerShell 不是“换了名字的 Bash”。cmdlet 常按“动词-名词”命名，参数常用 -Name 形式。
管道只建立“文本与带属性的对象”这一层直觉，不展开语法；PowerShell 调用外部程序时另有字节流和版本差异。
别名示例限定 Windows 上的默认 PowerShell 环境，用户配置可改变别名；不宣称所有平台都有相同别名。
ls -a 与 Get-ChildItem -Force 只比较显示隐藏项的常见意图，不宣称行为完全等价。
参考：https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_aliases
参考：https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pipelines
-->

---

## Coreutils for Windows

<p>想在 Windows 原生环境中使用 Unix 风格工具？可选装微软维护的 <strong>Coreutils for Windows</strong>。</p>
<p class="small">基于 uutils，打包 coreutils、findutils 与 grep，提供 <code>ls</code>、<code>cat</code>、<code>cp</code> 等工具。</p>

```sh
winget install Microsoft.Coreutils
```

<table class="compact mt-4">
  <thead><tr><th>注意</th><th>边界</th></tr></thead>
  <tbody>
    <tr><td>使用 PowerShell 时</td><td>要求 7.4+，推荐 7.6+；不要当作 Windows PowerShell 5.1</td></tr>
    <tr><td>同名命令可能冲突</td><td>Shell、别名与 PATH 都可能影响实际执行哪个命令</td></tr>
    <tr><td>工具集，不是 Linux 环境</td><td>不会把 PowerShell 变成 Bash；路径、权限等仍有差异</td></tr>
  </tbody>
</table>

<p class="small muted">项目与安装说明：<a href="https://github.com/microsoft/coreutils">microsoft/coreutils</a>；各工具支持 <code>--help</code>。</p>

<!--
用时约 1.5 分钟。推荐给希望在 Windows 原生环境复用 Unix 风格命令的候选人，不要求现场安装，也不修改别名或 Shell 配置。
安装命令和版本要求以项目 README 为准，授课前复核；不是 GNU coreutils 官方 Windows 构建，而是微软维护的 uutils 打包。
项目通过 PSReadLine 集成交互输入改写，但不会删除 PowerShell 别名。因此 Get-Command ls / Get-Help ls 仍可能显示别名，不能据此断定改写后实际调用的工具。
不承诺 Linux 脚本原样运行，不把工具集当作 WSL 或完整 Linux 环境的替代品。
参考：https://github.com/microsoft/coreutils
-->

---

# <Counter /> 遇到陌生命令怎么办

<table class="compact">
  <thead><tr><th>要确认什么</th><th>以 Git 为例</th></tr></thead>
  <tbody>
    <tr><td>用途与来源</td><td>先确认是哪个工具，不执行来历不明的命令</td></tr>
    <tr><td>本机版本</td><td><code>git --version</code> 或 <code>git -v</code></td></tr>
    <tr><td>总体用法</td><td><code>git --help</code> 或 <code>git -h</code></td></tr>
    <tr><td>具体功能的用法</td><td><code>git status -h</code>：简要帮助；<code>git help status</code>：手册</td></tr>
  </tbody>
</table>

<p class="no-margin"><strong>读帮助：</strong>用途 → 调用格式 → 必需参数 → 选项说明 → 示例。</p>
<p class="small">常见记号：<code>[…]</code> 表示可选项，<code>&lt;…&gt;</code> 表示必选项，<code>...</code> 常表示可重复，<code>|</code> 表示“或”。</p>

```text
usage: git [-v | --version] [-h | --help] [-C <path>] [-c <name>=<value>] [--exec-path[=<path>]] [--html-path] [--man-path]
           [--info-path] [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--bare] [--git-dir=<path>]
           [--work-tree=<path>] [--namespace=<name>] [--config-env=<name>=<envvar>] <command> [<args>]
```

<!--
用时约 2 分钟。现场展示 git --version、git --help、git status -h，无需初始化仓库。
git help status 可能打开浏览器或手册分页器，也可能因未安装手册而失败；可回到简要帮助或官方文档。
若进入常见的 less 分页器，用 q 退出；不把这当成所有交互程序的通用退出键。
不要为“试试看”运行会删除、覆盖文件或修改系统配置的示例。
-->

---

# <Counter /> 在终端里操作

<table class="compact mt-4">
  <thead><tr><th>操作</th><th>用途</th><th>习惯与边界</th></tr></thead>
  <tbody>
    <tr><td><kbd>↑</kbd> / <kbd>↓</kbd></td><td>翻阅历史命令</td><td>调回后可以编辑；检查后再按 <kbd>Enter</kbd></td></tr>
    <tr><td><kbd>Tab</kbd></td><td>尝试补全命令或路径</td><td>有多个候选时，继续输入；具体行为依 Shell 而异</td></tr>
    <tr><td><kbd>Ctrl</kbd>+<kbd>C</kbd></td><td>取消当前输入，或请求中断前台程序</td><td>不是撤销，已经发生的修改不会自动恢复</td></tr>
    <tr><td>清屏</td><td>Bash：<code>clear</code><br>PowerShell：<code>Clear-Host</code></td><td>整理显示，不是删除命令历史</td></tr>
  </tbody>
</table>

<p class="small muted">快捷键可能被终端或交互程序接管；<kbd>Ctrl</kbd>+<kbd>C</kbd> 不保证所有程序都立即退出。</p>

<!--
用时约 1.5 分钟。在命令提示符处演示上键调回、编辑和 Tab 补全；避免引入新工具。
输入一条不执行的命令，按 Ctrl+C 取消；解释运行中的程序也通常可请求中断，但不等于撤销操作。
复制粘贴快捷键依终端设置而异，不把 Ctrl+C 教成通用复制键。
-->

---
layout: section
---

# <Counter :level="1" /> 文件与文件系统

<p>从文件内容到路径定位</p>

<!--
本章 10 分钟｜累计 00:35–00:45
-->

---

# <Counter /> 文件类型

## <Counter :level="3" /> 扩展名提示文件格式，但不决定

<div class="two-col">
  <div class="stack">
    <div class="diagram-box text-red"><code>hello.txt</code></div>
    <div class="diagram-box text-peach"><code>hello.png</code></div>
    <div class="diagram-box text-green"><code>hello</code></div>
  </div>
  <div class="callout text-pink">
    <p>把 <code>.txt</code> 改成 <code>.png</code>，通常只改了名字，没有把内容转换成图片。</p>
    <p class="mt-3">软件可能综合扩展名、文件头和内容来判断格式。</p>
  </div>
</div>

---

<div class="flow">
  <div class="diagram-box text-pink">磁盘上的字节（二进制数据）</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-mauve">格式约定</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">软件解释与呈现</div>
</div>

<br>
<br>

<div class="card-grid five">
  <div class="card text-green"><h3>纯文本</h3><p>按字符编码解释的内容。</p></div>
  <div class="card text-sky"><h3>图片</h3><p>按图像格式解释像素与元数据。</p></div>
  <div class="card text-blue"><h3>音视频</h3><p>按音频视频格式解释声音画面。</p></div>
  <div class="card text-pink"><h3>复合文档</h3><p>多个资源共同描述内容与布局。</p></div>
  <div class="card text-red"><h3>可执行文件</h3><p>可被操作系统直接运行的程序。</p></div>
</div>

---

## <Counter :level="3" /> 示例：`.docx` 是一种复合文档格式

<div class="flow">
  <div class="diagram-box text-pink"><code>document.docx</code></div>
  <div class="flow-arrow">≈</div>
  <div class="diagram-box text-peach">ZIP 容器</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">XML、关系文件、媒体</div>
</div>

<div class="two-col">
  <div class="card text-blue"><h3>文字与排版</h3><p>通常由 XML 描述文本、样式和关系。</p></div>
  <div class="card text-mauve"><h3>嵌入资源</h3><p>图片等媒体可以作为内部文件保存。</p></div>
</div>

<p class="small muted">这是 <code>.docx</code> 的结构，不代表所有 Word 文件都相同；旧式 <code>.doc</code> 是另一种格式。</p>

---

## <Counter :level="3" /> 代码文件：纯文本

<div class="card-grid">
  <div class="card text-green">
    <h3>保存的是字符序列</h3>
    <p>源代码、Markdown、HTML、XML 都可以是纯文本。</p>
  </div>
  <div class="card text-blue">
    <h3>编辑器只是工具</h3>
    <p>同一份纯文本可以被不同编辑器打开和修改，不绑定某个软件。</p>
  </div>
</div>

<p class="lead">IDE 能提供导航、补全和检查，但它不是源码存在的前提。</p>

---

## <Counter :level="3" /> 常见的纯文本编辑器

<div class="two-col">
  <div class="card text-blue">
    <h3>Windows · 记事本</h3>
    <p>系统自带，改一个 <code>.txt</code> 最方便的方式。</p>
    <p class="mt-3">保存时确认保存类型是 <code>.txt</code>。</p>
  </div>
  <div class="card text-green">
    <h3>macOS · TextEdit</h3>
    <p>系统自带，但新建的是富文本文档。</p>
    <p class="mt-3">先「格式 → 制作纯文本」，再写 <code>.txt</code>。</p>
  </div>
</div>

<div class="callout text-mauve mt-5">
  <p>第三方编辑器：</p>
  <ul>
    <li>GUI 编辑器：<code>VS Code</code>、<code>Sublime Text</code>、<code>Notepad++</code></li>
    <li>终端编辑器：<code>nano</code>、<code>vim</code></li>
  </ul>
</div>

---

# <Counter /> 文件系统：目录树

<div class="filesystem-trees">
  <section>
    <h3>Windows</h3>
    <FilesystemTree os="windows" />
    <p class="filesystem-system-note"><code>Windows</code>：系统资源，<code>Program Files</code>：应用</p>
  </section>
  <section>
    <h3>Linux</h3>
    <FilesystemTree os="linux" />
    <p class="filesystem-system-note"><code>etc</code>：系统配置，<code>usr/bin</code>：程序</p>
  </section>
  <section>
    <h3>macOS</h3>
    <FilesystemTree os="macos" />
    <p class="filesystem-system-note"><code>System</code>：系统，<code>Applications</code>：应用</p>
  </section>
</div>

<div class="small">
  <p class="filesystem-hidden">以上都是多用户系统。用户 Alice 和 Bob 各有自己的用户主目录（家目录）。</p>
  <p class="filesystem-hidden">Linux 和 macOS 中，以 <code>.</code> 开头的文件或目录是隐藏项。</p>
  <p class="filesystem-hidden">Linux 和 macOS 都是单根文件系统，而 Windows 是多根文件系统（不同盘符就是不同的根）。</p>
  <p class="filesystem-hidden">在 Linux 中，文件和目录的名称区分大小写；而 Windows 和 macOS 不区分大小写。</p>
</div>

---

## <Counter :level="3" /> 在文件系统中导航

<div class="two-col">
  <div class="card text-pink">
    <h3>绝对路径</h3>
    <p>从固定的根位置开始。无论当前目录在哪里，指向都不变。</p>
    <p class="mt-3"><code>/usr/bin/git</code></p>
    <p class="mt-3"><code>/home/Alice/projects/hello.py</code></p>
    <p class="mt-3"><code>~/courses/MAT2001/Homework1</code></p>
    <p class="mt-3"><code>~/courses/MAT2001/../CSC3100/Homework1</code></p>
    <p class="mt-3"><code>"~/courses/GFH1000/Midterm Essay"</code></p>
  </div>
  <div class="card text-peach">
    <h3>相对路径</h3>
    <p>以当前目录为起点。同一个名字可能在不同位置指向不同文件/目录。</p>
    <p class="mt-3"><code>./hello.txt</code></p>
    <p class="mt-3"><code>hello.txt</code></p>
    <p class="mt-3"><code>./Homework1/main.py</code></p>
    <p class="mt-3"><code>Homework1/main.py</code></p>
    <p class="mt-3"><code>../../Bob/projects</code></p>
  </div>
</div>

<p><code>..</code> 表示上一级目录，<code>.</code> 表示当前目录，<code>~</code> 表示当前用户的家目录。</p>

---

## <Counter :level="3" /> 常用 Shell 命令

<div class="terminal-grid">
  <div>
    <div class="terminal-label text-blue">Windows · PowerShell</div>

```powershell
Get-Location                      # 显示当前目录
Get-ChildItem                     # 列出当前目录的文件和子目录

Set-Location projects/homework1   # 切换到指定目录（相对路径）
Get-Location

Set-Location ..                   # 切换到上一级目录
Get-Location

Set-Location ~/courses/ECE3080    # （绝对路径）
Get-Location

Get-ChildItem ~                   # 列出家目录的文件和子目录
Get-ChildItem -Force ~            # 包括隐藏文件
```

  </div>
  <div>
    <div class="terminal-label text-green">macOS / Linux · Bash</div>

```bash
pwd                    # print working directory
ls                     # list files and directories

cd projects/homework1  # change directory
pwd

cd ..
pwd

cd ~/courses/ECE3080
pwd

ls ~                   # list files and directories in home
ls -a ~                # list all files, including hidden ones
```

  </div>
</div>

许多 Bash 命令十分经典，PowerShell 也常常提供别名来兼容 Bash 的命令名称（但参数和含义不一定完全等价）。

---
layout: section
---

# <Counter :level="1" /> 环境变量，PATH

<!--
本章 10 分钟｜累计 00:45–00:55
-->

---

# <Counter /> Shell 变量

<table class="compact mt-4">
  <thead><tr><th>操作</th><th>Bash / Zsh</th><th>PowerShell</th></tr></thead>
  <tbody>
    <tr><td>赋值</td><td><code>name="RM"</code></td><td><code>$name = "RM"</code></td></tr>
    <tr><td>取值并输出</td><td><code>echo "$name"</code></td><td><code>echo $name</code></td></tr>
    <tr><td>赋值时注意</td><td>不写 <code>$</code>；等号两侧不能有空格</td><td>写 <code>$name</code>；等号两侧可以有空格</td></tr>
  </tbody>
</table>

<p>两边都输出 <code>RM</code>。修改变量的值，后续取到的值也随之改变。</p>
<p class="lead">普通 Shell 变量不会自动作为环境变量传给新启动的程序。</p>
<p class="small muted">Bash / Zsh 中用双引号包住 <code>"$name"</code>，可避免含空格的值被意外拆开。</p>

<!--
用时约 1 分钟。只讲名字、赋值和取值，不讲类型、数组或作用域。
示例使用新建的普通变量；Bash 中已导出的变量再次赋值仍保留导出属性，不把“不自动传递”讲成绝对不能传递。
echo "$name" 是 Shell 先取值再作为参数传入，与程序自行读取环境变量不同。
Zsh 默认的参数拆分行为与 Bash 不完全相同，双引号作为通用习惯介绍。
-->

---

# <Counter /> 环境变量：传给新启动的程序

<p>程序启动时会继承父进程的环境变量：一组<strong>名称与字符串值</strong>。</p>

<table class="compact mt-4">
  <thead><tr><th>操作</th><th>Bash / Zsh</th><th>PowerShell</th></tr></thead>
  <tbody>
    <tr><td>设置并传给子程序</td><td><code>export VAR="RM"</code></td><td><code>$env:VAR = "RM"</code></td></tr>
    <tr><td>查看一个值</td><td><code>printenv VAR</code></td><td><code>$env:VAR</code></td></tr>
    <tr><td>列出环境变量</td><td><code>printenv</code></td><td><code>Get-ChildItem Env:</code></td></tr>
  </tbody>
</table>

<p class="small">Bash / Zsh 用 <code>export</code> 标记变量供子程序继承；PowerShell 用 <code>$env:</code>，与普通变量 <code>$name</code> 区分。</p>
<p class="lead">这里的设置不永久保存，也不会同步修改已经运行的其他程序。</p>
<p class="small muted">修改影响当前 Shell 及其后续启动的子程序。列出的环境变量可能包含令牌等凭据，不要直接截图或公开粘贴。</p>

<!--
用时约 1.5 分钟。环境变量并非只能由 Shell 创建，Shell 本身也从父进程继承环境；这里只教在 Shell 中查看与设置。
可先尝试 VAR="RM" 再 printenv VAR，对比 export 后的结果；确保演示变量此前未导出。
继承是启动时复制，不是进程间共享一个实时更新的变量。子程序的修改也不会反向修改父 Shell。
不讲 shell 配置文件、系统设置面板、setx 或持久化配置。
参考：https://www.gnu.org/software/bash/manual/html_node/Environment.html
参考：https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables
-->

---

## 环境变量有什么用？给程序传配置

<p>先按上一页设置 <code>VAR="RM"</code>，再从<strong>同一个 Shell</strong> 启动 Python：</p>

```bash
python3 -c "import os; print('Hello,', os.getenv('VAR'))"
```

```text
Hello, RM
```

<p><strong>Shell 设置配置 → Python 读取环境变量 → 改变问候的名字。</strong></p>

<p class="small">把值改成 <code>RoboMaster</code> 后重新运行，输出变为 <code>Hello, RoboMaster</code>；未设置时使用默认值 <code>None</code>。</p>
<p class="small"><code>VAR</code> 是本例约定的名字，不是系统内置功能。<strong>变量的作用由读取它的程序决定。</strong></p>
<p class="small muted">Windows 上若使用 <code>python</code>，替换命令名即可。接下来：真实的环境变量 <code>PATH</code> 如何参与查找程序？</p>

<!--
用时约 1.5 分钟。沿用前面 python -c 的形式，不要求理解 Python 模块机制。
os.getenv 读取 Python 进程自己的环境变量；第二个参数是变量不存在时的默认值。
命令中的 VAR 没有 $，不是 Shell 将其替换成值，而是 Python 主动按名字读取。
示例输出依赖上一页的设置；只设置普通变量不会自动获得同样结果。
参考：https://docs.python.org/3/library/os.html#os.getenv
-->

---

# <Counter /> PATH：搜索程序的位置

<div class="two-col">
  <div class="callout text-red">
  <p>输入完整路径：明确指定启动哪个程序。</p>

<p class="small muted">Bash：</p>

```bash
/usr/bin/git --version
```

<p class="small muted">PowerShell （反引号转义空格或使用 Call Operator 调用）：</p>

```powershell
C:\Program` Files\Git\bin\git.exe --version
& "C:\Program Files\Git\bin\git.exe" --version
```

  </div>
  <div class="callout text-blue">
  <p>只输入名称：交给 Shell 按当前环境寻找。</p>

<p class="small muted">Bash：</p>

```bash
git --version
```

<p class="small muted">PowerShell：</p>

```powershell
git --version
git.exe --version
```

  </div>
</div>

<br>

```bash
printenv PATH          # 显示 PATH 的值
```

<div class="flow">
  <div class="diagram-box text-mauve"><code>git</code></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-pink">PATH 位置 1</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">PATH 位置 2</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">直到找到可执行程序</div>
</div>

---

## 目标程序到底命中了哪里？

<div class="terminal-grid">
  <div>
    <div class="terminal-label text-blue">Windows · PowerShell</div>

```powershell
Get-Command git
where.exe git
```

  </div>
  <div>
    <div class="terminal-label text-green">macOS / Linux · Bash/Zsh</div>

```bash
command -v git
where git       # Zsh built-in command
which git
```

  </div>
</div>

`PATH` 是有序的，Shell 会按顺序查找每个目录，直到找到第一个可执行文件。

如果有多个同名程序，Shell 只会执行第一个找到的。

<v-click>
【思考】如果你有两个项目，一个使用 Python 3.9，另一个使用 Python 3.11，如何在同一台电脑上同时安装并切换使用？
</v-click>

<!--
Google 搜索一下 pyenv，展示 AI 概览对其的解释：pyenv 通过修改环境变量 `PATH`，把自己的垫片（shims）路径放在最前面。
-->

---
layout: section
---

# <Counter :level="1" /> 环境与依赖管理

<!--
本章 20 分钟｜累计 00:55–01:15（含演示）
重点：隔离与复现；各语言工具概览快讲。
-->

---

## 一段 Python 代码的依赖

<div class="two-col">
  <div>

```python
# 标准库：随 Python 提供
import math
import os

# 第三方库：需要额外安装
import numpy as np
import pandas as pd

print("Hello from", os.name, "with Python", os.sys.version)

values = np.array([1, 4, 9], dtype=float)
table = pd.DataFrame({"value": values})
table["root"] = np.sqrt(table["value"])

print(table)
print(f"Total: {math.fsum(table['root']):.1f}")
```

  </div>
  <div>
    <div class="card text-pink">
      <h4>解释器</h4>
      <p>Python 解释器负责读取并执行这段代码。</p>
    </div>
    <div class="card text-mauve mt-3">
      <h4>标准库</h4>
      <p><code>math</code>、<code>os</code> 随 Python 提供，不需要单独安装。</p>
    </div>
    <div class="card text-peach mt-3">
      <h4>第三方包</h4>
      <p><code>numpy</code>、<code>pandas</code> 不随 Python 提供，需要安装到正确环境。</p>
    </div>
  </div>
</div>

---

## 版本也是环境要求的一部分

<div class="two-col">
  <div class="card text-blue">
    <h3>项目要求</h3>
    <p><code>Python &gt;= 3.12, &lt; 3.14</code></p>
    <p><code>numpy &gt;= 2, &lt; 3</code></p>
    <p><code>pandas &gt;= 2, &lt; 3</code></p>
    <p class="small mt-3">描述可以接受的版本范围。</p>
  </div>
  <div class="card text-red">
    <h3>实际环境</h3>
    <p><code>Python 3.12.x</code></p>
    <p><code>numpy 2.x</code></p>
    <p><code>pandas 2.x</code></p>
    <p class="small mt-3">最终需要选出具体且相互兼容的版本。</p>
  </div>
</div>

<p class="lead muted">这里的 Python 版本包括解释器及其对应的标准库。</p>
<p class="lead">版本匹配是确保代码在不同环境中一致运行的关键。</p>

---

# <Counter /> 传统的 Python 包管理方案

## <Counter :level="3" /> 系统级 pip

```bash
pip install numpy pandas
```

<div class="card-grid mt-5">
  <div class="card text-red">
    <h3>环境耦合</h3>
    <p>多个项目共享系统 Python 和同一组已安装包，版本要求可能互相冲突。</p>
  </div>
  <div class="card text-peach">
    <h3>难以同步环境</h3>
    <p>安装状态藏在这台机器里；其他机器无法仅凭源码知道需要哪些包和版本。</p>
    <p>手动一个一个地安装和管理非常繁琐。</p>
  </div>
</div>

<p class="small muted">终端里名为 <code>pip</code> 的命令也可能指向不同的 Python 环境，需要能够检查它实际调用了什么。</p>

<!--
如何解决这两个问题呢？
-->

---

## <Counter :level="3" /> 虚拟环境 venv：给项目一个隔离环境

<div class="terminal-grid">
  <div>
    <div class="terminal-label text-blue">创建、激活、安装</div>

```bash
python -m venv .venv
source .venv/bin/activate
pip install numpy pandas
```

  </div>
  <div>
    <div class="terminal-label text-green">检查实际调用路径</div>

```text
$ command -v python
.../project/.venv/bin/python

$ command -v pip
.../project/.venv/bin/pip
```

  </div>
</div>

注意：Windows PowerShell 中的路径、指令等有所不同。

<div class="callout text-mauve mt-4">
  <p>路径指向 <code>.venv</code>，说明当前项目使用的是隔离环境，而不是系统级 Python。</p>
</div>

<!--
## 实操展示

```powershell
where.exe python
where.exe pip
pip list

ls
python -m venv .venv
ls

.\.venv\Scripts\activate

where.exe python
where.exe pip
printenv PATH
pip list

deactivate        # 提问：为什么这里不用像 activate 一样写完整路径？

printenv PATH
where.exe pip
pip list
```
-->

---

## <Counter :level="3" /> `requirements.txt` 依赖清单

<div class="three-col">
  <div>
    <div class="terminal-label text-green">1. 在当前环境中导出</div>

```bash
pip freeze > requirements.txt
```

  </div>
  <div>
    <div class="terminal-label text-peach">2. 形成一份依赖清单</div>

```text
# requirements.txt
numpy==2.5.3
pandas==3.0.6
python-dateutil==2.9.0.post0
six==1.17.0
tzdata==2026.4
```

  </div>
  <div>
    <div class="terminal-label text-green">3. 在目标环境中安装</div>

```bash
pip install -r requirements.txt
```

  </div>
</div>

<div class="card-grid mt-5">
  <div class="card text-blue"><h3>它描述什么？</h3><p>项目需要哪些 Python 包，以及允许或要求什么版本。</p></div>
  <div class="card text-red"><h3>它没有描述什么？</h3><p>不会自动提供隔离环境，也不会完整描述 Python、编译器和系统库。</p></div>
</div>

---

## <Counter :level="3" /> Python 版本的管理

如果不同项目需要不同的 Python 版本，系统级 Python 和 venv 都无法自动切换。

一些解决方案：

- pyenv：在同一台机器上安装多个 Python 版本，并在不同项目间切换。
- asdf：支持多种语言的版本管理器，类似 pyenv，但可以管理更多语言。
- Python Install Manager：微软和 Python 官方提供的 Windows 安装器，支持安装多个版本并切换。

---

# <Counter /> 更现代的解决方案

## <Counter :level="3" /> Conda：适用于数据科学的跨平台环境管理器

```bash
conda create --name myenv-A python=3.11 numpy pandas
conda activate myenv-A
```

```bash
conda create --name myenv-B python=3.13 ruff pyright requests
conda activate myenv-B
```

<div class="card-grid two mt-4">
  <div class="card text-green">
    <h3>专攻数据科学</h3>
    <p>可以把 Python、数值计算包和部分底层库一起放进一个跨平台环境。</p>
  </div>
  <div class="card text-red">
    <h3>环境增加后，管理困难</h3>
    <p>需要记住名称、手动切换、确认当前环境，并定期清理不再使用的环境。</p>
  </div>
</div>

<p class="lead">少量、明确命名的环境很方便；项目和环境数量增加后，环境映射与切换的管理成本也会增加。</p>

<!--
`environment.yml` 可以记录环境和依赖，便于分享与重建；Conda 也支持把环境显式放到项目目录，但这些做法仍需要额外维护。
-->

---

## <Counter :level="3" /> 更“软件工程”的方案

<div class="three-col">
  <div class="card text-blue">
    <h3>项目描述</h3>
    <p>把项目名称、Python 要求和直接依赖写进 <code>pyproject.toml</code>。（遵循 PEP 621 规范）</p>
  </div>
  <div class="card text-mauve">
    <h3>环境管理</h3>
    <p>工具可以创建或使用项目隔离环境，减少手工切换和遗漏。</p>
  </div>
  <div class="card text-green">
    <h3>可重建信息</h3>
    <p>锁文件保存更具体的版本选择，方便团队和 CI 使用同一组结果。</p>
  </div>
</div>

<div class="callout text-peach mt-5">
  <p>常见工具：Poetry、PDM、uv 等</p>
</div>

---

## uv：几条命令建立项目环境

<div class="two-col">
  <div class="card text-blue">
    <h4>创建项目</h4>

```bash
uv init
uv add numpy
```

  <p class="small">生成 <code>pyproject.toml</code> 等项目文件，并创建虚拟环境、添加依赖。</p>
  </div>
  <div class="card text-mauve">
    <h4>安装依赖、同步环境</h4>

```bash
uv sync
```

  <p class="small">根据 <code>pyproject.toml</code> 和 <code>uv.lock</code> 解析依赖、安装到隔离环境。</p>
  </div>
</div>

<table class="compact mt-5">
  <thead><tr><th>项目中的东西</th><th>uv 维护的内容</th></tr></thead>
  <tbody>
    <tr><td><code>pyproject.toml</code> 项目描述</td><td>项目要求、Python 版本和直接依赖的版本信息</td></tr>
    <tr><td><code>uv.lock</code> 锁文件</td><td>解析得到的更具体版本信息</td></tr>
    <tr><td><code>.venv/</code> 虚拟环境</td><td>当前项目实际使用的隔离环境</td></tr>
  </tbody>
</table>

<p class="lead">把“安装什么、用哪个版本、环境在哪里”变成项目可以携带的描述。</p>

---

# <Counter /> C/C++：工具链和库

<div class="flow">
  <div class="diagram-box text-pink">编译器<br>gcc / clang / MSVC</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-mauve">链接器与构建系统<br>linker / CMake</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-peach">第三方库<br>头文件、库文件、ABI</div>
</div>

<div class="card-grid mt-4">
  <div class="card text-red">
    <h3>原始方案（但仍然常见）</h3>
    <p>直接把库的源码复制进项目，例如放在 <code>third_party/</code>，再和项目一起构建。</p>
  </div>
  <div class="card text-blue">
    <h3>包管理器方案</h3>
    <p>系统包管理器、vcpkg、Conan 等都能帮忙，但平台、构建系统、ABI 和分发方式容易碎片化。</p>
  </div>
</div>

<p class="lead">C/C++ 的环境管理原始而复杂，需要手动处理编译器、链接器、构建系统等多个环节。</p>

<i class="muted">所以我们放弃了 C/C++</i>

---

# <Counter /> Rust：Cargo 带来现代化体验

<div class="two-col">
  <div class="card text-blue">
    <h3>Python + uv</h3>
    <p><code>pyproject.toml</code> 描述项目，<code>uv.lock</code> 固定解析结果，<code>.venv/</code> 承载隔离环境。</p>
  </div>
  <div class="card text-peach">
    <h3>Rust + Cargo</h3>
    <p><code>Cargo.toml</code> 描述项目，<code>Cargo.lock</code> 固定解析结果，<code>target/</code> 保存构建结果。</p>
  </div>
</div>

<div class="card-grid mt-5">
  <div class="card text-mauve"><h3>工具链</h3><p>C/C++ 需要组合编译器、链接器和构建系统；Rust 通常由 rustup 提供工具链，Cargo 负责项目构建。</p></div>
  <div class="card text-green"><h3>依赖体验</h3><p>Cargo 把清单、依赖解析、构建和运行放进一套约定，体验上类似 uv 带来的现代化工作流。</p></div>
</div>

---

<br>
<br>
<br>
<br>
<br>
<br>

<div class="flow">
  <div class="diagram-box text-pink">工具链</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-peach">库依赖</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-blue">源代码</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-sky">构建规则</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">可重建的环境</div>
</div>

<p class="lead text-center">一个可靠的工程项目不是 <i>It runs on my machine</i></p>

---
layout: section
---

# <Counter :level="1" /> 开发规范与代码质量管理

<p>从“能运行”到“可协作、可维护、可验证”</p>

<!--
本章 14 分钟｜累计 01:15–01:29
报错示例讲用途，不逐行展开。
-->

---

<h2>代码质量检查的几种类型</h2>
<table class="compact">
  <thead><tr><th></th><th>内容</th><th>具体示例</th></tr></thead>
  <tbody>
    <tr><td><strong>Formatting</strong></td><td>排版与代码风格</td><td>缩进、空格、换行、命名等可见形式</td></tr>
    <tr><td><strong>Type / Build Checking</strong></td><td>语言/类型系统的一致性，构建的正确性</td><td>类型、名称、借用、依赖与构建条件</td></tr>
    <tr><td><strong>Linting</strong></td><td>静态规则与可疑模式</td><td>规则、反模式、无效代码、include / import 等</td></tr>
    <tr><td><strong>Testing</strong></td><td>运行时的实际行为</td><td>选定输入和环境下运行后的输出、状态</td></tr>
    <tr><td><strong>Code Review</strong></td><td>人工评审</td><td>自动工具不了解的上下文、风险与可维护性</td></tr>
  </tbody>
</table>

---

# <Counter /> Formatting：统一代码风格

### PEP 8 – Style Guide for Python Code

<div class="two-col">
  <div >
    <ul>
      <li>使用 4 个空格缩进，不混用 Tab。</li>
      <li>函数和变量：<code>snake_case</code>，类名：<code>CapWords</code>。</li>
      <li>逗号、运算符和括号附近保持一致的空格。</li>
      <li>import 集中在顶部，顶层定义之间保留空行。</li>
      <li>过长的表达式主动换行，让结构可读。</li>
      <li>...</li>
    </ul>
  </div>
  <iframe src="https://peps.python.org/pep-0008/#pet-peeves" width="100%" height="350"></iframe>
</div>

<!--
VS Code 等编辑器可以依据配置，自动进行空格和 tab 之间的转换。
-->

---

## 代码风格是团队协作的共同约定

<div class="card-grid three">
  <div class="card text-pink"><h3>减少噪声</h3><p>统一排版后，diff 更小，评审可以集中在逻辑变化。</p></div>
  <div class="card text-peach"><h3>减少争论</h3><p>格式交给工具和配置，不把个人偏好变成评审阻塞。</p></div>
  <div class="card text-blue"><h3>降低协作成本</h3><p>新成员能更快读懂代码，也更少制造无意义的合并冲突。</p></div>
</div>

<div class="two-col mt-4">
  <div class="card text-green">
    <h3>常见 Formatter</h3>
    <p>Python：Ruff formatter、Black、autopep8<br>Rust：rustfmt<br>C/C++：clang-format</p>
  </div>
  <div class="card text-mauve">
    <h3>C/C++ 没有唯一标准</h3>
    <p>GNU、Google、LLVM、Linux 内核等组织都有自己的风格文档。项目需要明确内部约定，正说明风格规范本身很重要。</p>
  </div>
</div>

---

# <Counter /> Type / Build Checking：语法检查

<div class="two-col">
  <div class="card text-blue">
    <h3>Python：静态类型检查</h3>
    <p>结合类型标注和推断，检查参数、返回值和对象使用是否一致。</p>
    <p class="small">常见工具：Pyright、Mypy、Pyrefly</p>
  </div>
  <div class="card text-peach">
    <h3>Rust：编译器级检查</h3>
    <p><code>cargo check</code> 检查类型、名称、借用和项目编译条件，通常不生成最终可执行文件。</p>
    <p class="small">Rust 的编译器把许多检查整合进了语言工具链。</p>
  </div>
</div>

这一步关注“代码结构是否自洽、项目能否通过静态检查”，不验证业务行为。

---

```bash
uv run pyrefly check
```

```ansi
 INFO Checking project configured at `/home/Alice/Coding/server-project/pyproject.toml`
[31mERROR[0m Class `State` has no class attribute `RECOVER` [2m[missing-attribute][0m
   [1m[94m-->[0m app/core/atom.py:796:51
    [1m[94m|[0m
[1m[94m796 |[0m             events.append(self._record_transition(State.RECOVER, "operator reset", source))
    [1m[94m|[0m                                                   [1m[91m^^^^^^^^^^^^^[0m
    [1m[94m|[0m
  Did you mean `RECOVERY`?
[31mERROR[0m Argument `str | None` is not assignable to parameter `cmd_seq` with type `int` in function `sequencer.check` [2m[bad-argument-type][0m
   [1m[94m-->[0m app/core/gate.py:148:38
    [1m[94m|[0m
[1m[94m148 |[0m         seq_reason = sequencer.check(proposal.cell_id)
    [1m[94m|[0m                                      [1m[91m^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
  The declared type does not allow `None`. Consider narrowing the value with an `is not None` check.
 INFO 2 errors (43 suppressed, 2 warnings not shown)

```

---

# <Counter /> Linting：检查可疑写法和维护风险

<div class="two-col">
  <div class="card text-pink">
    <h3>与语法检查不同</h3>
    <p>Linting 更关心“这种写法是否容易出错、难以维护或违反项目约定”。</p>
  </div>
  <div class="card text-green">
    <h3>常见目标</h3>
    <p>未使用的代码、可疑条件、过时写法、重复逻辑、不必要的 include / import，以及易隐藏 bug 的模式。</p>
  </div>
</div>

### 常见 Linter

- Python：Ruff、Pylint、Flake8
- C/C++：clang-tidy、include-what-you-use（IWYU）
- Rust：Clippy

---

```bash
uv run ruff check .
```

```ansi
[1m[91mE711 [0m[1mComparison to `None` should be `cond is None`[0m
   [1m[94m-->[0m app/cli.py:99:21
    [1m[94m|[0m
[1m[94m 97 |[0m     atom = build_atom(log_path=log, start_percent=start_percent)
[1m[94m 98 |[0m     y_axis_id = atom._config.y_axis_id
[1m[94m 99 |[0m     if y_axis_id == None:
    [1m[94m|[0m                     [1m[91m^^^^[0m
[1m[94m100 |[0m         typer.echo("this cell has no Y axis configured; ALIGN_2D needs one")
[1m[94m101 |[0m         raise typer.Exit(code=2)
    [1m[94m|[0m
[1m[96mhelp[0m: [1mReplace with `cond is None`[0m

Found 1 error.
No fixes available (1 hidden fix can be enabled with the `--unsafe-fixes` option).
```

---

# <Counter /> Testing：用运行结果验证行为

<div class="card-grid">
  <div class="card text-pink"><h3>单元测试</h3><p>验证一个函数、模块或小对象，通常隔离外部依赖。</p></div>
  <div class="card text-peach"><h3>集成测试</h3><p>验证多个模块、进程或真实依赖协作时的接口。</p></div>
  <div class="card text-blue"><h3>系统 / 端到端测试</h3><p>从用户或协议入口观察完整程序的行为。</p></div>
  <div class="card text-green"><h3>HiL 测试</h3><p>Hardware-in-the-Loop：软件连接真实或仿真的硬件环境，验证控制链路。</p></div>
</div>

<p>静态检查可以发现某些结构问题，但不替代测试；测试覆盖了选定场景，也不能证明没有遗漏的输入、环境和时序。</p>

---

## 为了便于测试，代码要留下“接缝”

<div class="two-col">
  <div class="card text-blue">
    <h3>隔离外部依赖</h3>
    <ul>
      <li>用 mock、stub 或 fake 替代网络、硬件、时钟等依赖。</li>
      <li>通过依赖注入，让测试可以传入替身。</li>
      <li>不要让每个单元测试都必须启动完整系统。</li>
    </ul>
  </div>
  <div class="card text-green">
    <h3>设计可测试的模块</h3>
    <ul>
      <li>模块化、职责清晰、低耦合。</li>
      <li>把纯计算逻辑与 I/O、设备和网络边界分开。</li>
      <li>让测试独立、可重复，并覆盖正常、边界、错误和回归场景。</li>
    </ul>
  </div>
</div>

<p class="lead">“难以写测试”也是一种提醒：代码的依赖关系和职责边界还不够清楚。</p>

---

### 常见测试框架

- Python：pytest、unittest
- C++：Google Test、Catch2

```bash
uv run pytest
```

```ansi
[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[31mF[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[32m.[0m[31m [100%]
============================================================= FAILURES =============================================================
[31m[1m_________________________________________________ test_stale_encoder_fault_via_api _________________________________________________[0m

client = <starlette.testclient.TestClient object at 0x7f4365397350>

    [0m[94mdef[39;49;00m[90m [39;49;00m[92mtest_stale_encoder_fault_via_api[39;49;00m(client: TestClient) -> [94mNone[39;49;00m:[90m[39;49;00m
        client.post([33m"[39;49;00m[33m/v1/simulation/faults[39;49;00m[33m"[39;49;00m, json={[33m"[39;49;00m[33mfault[39;49;00m[33m"[39;49;00m: [33m"[39;49;00m[33mSTALE_ENCODER[39;49;00m[33m"[39;49;00m})[90m[39;49;00m
        rejected = client.post([33m"[39;49;00m[33m/v1/motion/proposals[39;49;00m[33m"[39;49;00m, json=_valid_proposal())[90m[39;49;00m
>       [94massert[39;49;00m rejected.json()[[33m"[39;49;00m[33mrejection_reason[39;49;00m[33m"[39;49;00m] == [33m"[39;49;00m[33mSTALE_FEEDBACK[39;49;00m[33m"[39;49;00m[90m[39;49;00m
[1m[31mE       AssertionError: assert None == 'STALE_FEEDBACK'[0m

[1m[31mtests/test_api_faults.py[0m:90: AssertionError
[36m[1m===================================================== short test summary info ======================================================[0m
[31mFAILED[0m tests/test_api_faults.py::[1mtest_stale_encoder_fault_via_api[0m - AssertionError: assert None == 'STALE_FEEDBACK'
[31m[31m[1m1 failed[0m, [32m348 passed[0m[31m in 1.24s[0m[0m
```

---

# <Counter /> 质量管理需要工具和人一起维护

<div class="two-col">
  <div class="card text-blue">
    <h3>CI：重复执行约定</h3>
    <p>在干净、可重复的环境中运行检查，避免“我本机能过”成为唯一证据。</p>
    <p class="small">CI 只会执行配置中写明的检查，不会自动提高质量标准。</p>
  </div>
  <div class="card text-pink">
    <h3>Code Review：人工评审</h3>
    <p>检查需求是否满足、设计是否清楚、错误处理和测试是否充分，以及取舍是否值得长期维护。</p>
    <p class="small">工具擅长重复和明确的规则，人负责上下文、风险和责任。</p>
  </div>
</div>

<div class="flow mt-5">
  <div class="diagram-box text-peach">本地检查</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">CI 重跑</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-blue">人工评审</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-pink">可合并的变化</div>
</div>

---

# <Counter /> 检查不能包办全部质量

<div class="flow">
  <div class="diagram-box text-pink">Formatting</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">Type / Build Check</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-blue">Linting</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">Testing</div>
</div>

<div class="card-grid three mt-5">
  <div class="card text-pink"><h3>自动化</h3><p>尽早、重复、客观地发现一部分问题。</p></div>
  <div class="card text-peach"><h3>模块化</h3><p>让代码更容易隔离、验证和替换依赖。</p></div>
  <div class="card text-blue"><h3>协作</h3><p>用配置、CI 和人工评审把个人习惯变成团队约定。</p></div>
</div>

<p class="lead">接下来，Git 负责记录代码变化，让协作过程和项目历史可追踪。</p>

---
layout: section
---

# <Counter :level="1" /> Git 与版本控制

<p>把变化留下来，让尝试与协作有据可查</p>

<!--
本章 19 分钟｜累计 01:29–01:48
重点：提交、diff、分支与协作；后半补充工具快讲。
-->

---

# <Counter /> 版本控制，从手动存档开始

<p class="lead">改动之前，先留一份“还能用的版本”。</p>

<div class="git-archives">
  <div class="card text-blue"><strong>项目.zip</strong><span>第一次跑通</span></div>
  <div class="card text-peach"><strong>项目_修改版.zip</strong><span>加了些东西</span></div>
  <div class="card text-pink"><strong>项目_最终版.zip</strong><span>到底改了什么？</span></div>
  <div class="card text-pink"><strong>项目_最终最终版.zip</strong><span>这次真的行了</span></div>
</div>

<div class="two-col mt-6">
  <div class="callout text-green"><h3>它已经在做版本控制</h3><p>保留过去的状态，给自己留下退路。</p></div>
  <div class="callout text-peach"><h3>版本一多，就难管理</h3><p>哪份最新？两份差在哪？谁改的？<br>想拿回其中一个改动怎么办？</p></div>
</div>

---

# <Counter /> Git：分布式版本控制系统

## <Counter :level="3" /> 自主选择提交的时机

<div class="flow mt-6">
  <div class="diagram-box text-orange">修改、保存文件<br>
  <span class="small">&nbsp;&nbsp;&nbsp;可以反复很多次</span></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">一个有意义的改动完成<br>
  <span class="small">&nbsp;&nbsp;&nbsp;检查后 commit</span></div>
</div>

<p class="small muted">例如：“修正温度单位”与“补充安装说明”可以分别提交，方便以后理解和处理。</p>

### Commit（提交）：记录一次选定的改动

由你决定时机、范围，并写下 message，说明这次做了什么。

<p class="small mt-3">记录保存在本地 Git 仓库中，不需要联网。</p>

---
clicks: 2
---

## <Counter :level="3" /> 历史记录：既能看清变化，也方便回到过去

<p>每次提交留下版本、作者、时间和说明；可以比较，也可以回到已记录的版本。</p>

<GitHistory kind="history" :step="$clicks" />

<p class="small muted">图中的 A、B、C、D 是提交的简写。能回溯的是已提交且仍保留的历史。</p>

---

## <Counter :level="3" /> Diff：两个 Commit 之间具体改了什么内容？

```python {monaco-diff} {height:'auto'}
import banana


class Monkey:
    # Bananas the monkey can eat.
    capacity: int

    def eat(self, n: int) -> None:
        """Make the monkey eat n bananas!"""
        self.capacity -= n * banana.size
        return

    def feeding_frenzy(self) -> str:
        self.eat(9)
        return "Yum yum"

~~~
import banana


class Monkey:
    # Bananas the monkey can eat.
    # New feature: the monkey can eat fractional bananas!
    capacity: float

    def eat(self, n: float) -> None:
        """Make the monkey eat n bananas!"""
        self.capacity -= n * banana.size

    def feeding_frenzy(self) -> str:
        self.eat(9.25)
        return "Yum yum"

```

<div class="two-col mt-5">
  <div><h3>编辑时实时查看</h3><p>编辑器可在行旁标记增删改，随时展开差异。</p></div>
  <div><h3>提交前检查</h3><p>是否有误改、遗漏，或混入了另一件事？</p></div>
</div>

<!--
编辑器集成参考：https://code.visualstudio.com/docs/sourcecontrol/overview
这是离线可用的编辑器示意，不是特定产品的截图。
-->

---
clicks: 2
---

## <Counter :level="3" /> 分支：独立尝试，准备好再合并

<GitHistory kind="branch" :step="$clicks" />

<div class="callout text-peach mt-2">
  <p><strong>合并与冲突：</strong>能自动整合的变化由 Git 处理；无法自动决定时，Git 会标出冲突，提供处理与完成合并的机制，由人判断最终内容。</p>
</div>

---
clicks: 1
---

## <Counter :level="3" /> Revert：为撤销留下记录

<p>已经提交的某个改动不想要了，可以新增一次提交来撤销它。</p>

<GitHistory kind="revert" :step="$clicks" />

<p class="small muted">这里的文档改动与提示音互不依赖。复杂情况下，撤销也可能需要处理冲突。</p>

---
clicks: 1
---

## <Counter :level="3" /> Cherry-pick：只取需要的那次改动

<GitHistory kind="cherry-pick" :step="$clicks" />

<p class="small muted">实线表示历史关系；虚线表示应用改动。C′ 是新提交。选中的改动若依赖其他改动，仍需处理依赖或冲突。</p>

---

## <Counter :level="3" /> 多人协作

<div class="two-col">
  <div class="card text-blue"><h3>Alice · 日志分支</h3><p>增加日志 → 补充日志测试</p><p class="small mt-3">每次提交都围绕日志功能。</p></div>
  <div class="card text-peach"><h3>Bob · 界面分支</h3><p>调整布局 → 修复按钮状态</p><p class="small mt-3">每次提交都围绕界面功能。</p></div>
</div>

<div class="flow">
  <div class="diagram-box text-blue">各自推进、各自提交</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">审阅，再合并到主线</div>
</div>

<p>实时共享编辑适合同步讨论、一起写；Git 还支持<strong>异步开发、隔离未完成的工作、按主题审阅和撤销</strong>。</p>
<p class="small muted">不同人的不同改动可以分成清楚的 commit；分支不会自动消除冲突，仍然需要沟通。</p>

---
clicks: 3
---

## <Counter :level="3" /> Remote：在两个 Git 仓库之间同步

<GitHistory kind="remote" :step="$clicks" />

<p class="small muted">将另一个 Git 仓库添加为 remote，从而可以在两个仓库之间同步提交；添加 remote 不会开启自动同步 commit，而是在需要时手动执行同步操作。</p>

---

# <Counter /> GitHub：共同的远程仓库

<p><strong>Git 是版本控制工具；GitHub 是托管 Git 仓库的协作平台。</strong></p>

<div class="git-hub-map">
  <div class="card text-green git-hub-center"><h3>GitHub 上的团队仓库</h3><p>集中托管，作为共同的 remote</p></div>
  <div class="git-hub-link">↕ 同步提交与分支</div>
  <div class="git-hub-link">↕ 同步提交与分支</div>
  <div class="card text-blue"><h3>Alice 的本地仓库</h3><p>独立工作，按需发送和获取</p></div>
  <div class="card text-peach"><h3>Bob 的本地仓库</h3><p>独立工作，按需发送和获取</p></div>
</div>

<p class="small muted">有相应权限时，大家可以同步同一仓库。Git 不依赖 GitHub：也可以用 GitLab、Gitea 或自建 Git 服务。</p>

<!--
平台定位参考：https://docs.github.com/en/get-started/start-your-journey/what-is-github
图示采用团队成员都有权限的共同仓库，不引入 fork 工作流。
-->

---

## GitHub 的协作能力

<div class="card-grid three">
  <div class="card text-blue">
    <h3>Issues</h3>
    <p>记录问题、需求和讨论。</p>
    <ul class="small mt-3">
      <li>Bug report</li>
      <li>Feature request</li>
      <li>Question</li>
    </ul>
  </div>
  <div class="card text-peach">
    <h3>Pull Requests</h3>
    <p>提出合并请求，展示 diff，讨论和审阅改动。</p>
    <ul class="small mt-3">
      <li>“这组提交修复了断线提示，请检查。”</li>
      <li>“新功能实现：用户登录”</li>
    </ul>
  </div>
  <div class="card text-green">
    <h3>GitHub Actions</h3>
    <p>按配置自动运行构建、测试等检查。</p>
    <ul class="small mt-3">
      <li>CI/CD 流水线</li>
      <li>自动化工作流程</li>
    </ul>
  </div>
</div>

<div class="flow mt-6">
  <div class="diagram-box text-blue">讨论要做什么</div><div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">审阅具体改动</div><div class="flow-arrow">→</div>
  <div class="diagram-box text-green">检查后合并</div>
</div>

---

# <Counter /> Git 和 Coding Agents

<p>Vibe coding 时，Agent 可能一次改动很多文件。越容易生成改动，越需要看清和控制改动。</p>

<div class="git-agent-loop">
  <div class="card text-blue"><strong>01 · 留基线</strong><p>开始前，记录已知可用的状态。</p></div>
  <div class="card text-peach"><strong>02 · 看 diff</strong><p>Agent 改完后，检查它究竟动了哪里。</p></div>
  <div class="card text-pink"><strong>03 · 验证</strong><p>运行、测试，判断是否符合预期。</p></div>
  <div class="card text-green"><strong>04 · 作决定</strong><p>接受后按主题提交；失败则修正或撤销。</p></div>
</div>

<p class="small">许多 Agent 已接入 Git 工作流：Cursor 集成 diff 审阅，Claude Code 支持独立工作目录与 PR 流程。</p>
<p class="lead">有了 Git 进行版本控制，不仅方便人工审查改动，也便于 Agent 进行代码检查和追溯。</p>

<!--
一次对话不一定对应一次合适的提交；已提交的错误可用 revert，尚未提交的修改也要先检查范围再决定如何撤销。
Agent 自带 checkpoint 与 Git 历史不能一概视为同一种机制。
资料：https://cursor.com/docs/agent/overview
资料：https://code.claude.com/docs/en/common-workflows
-->

---

# <Counter /> Git 的缺陷：大型二进制资源

<div class="two-col">
  <div class="card text-green"><h3>代码、配置、Markdown</h3><p>通常是纯文本，按行 diff 能看清增删改，也便于合并。</p></div>
  <div class="card text-peach"><h3>图片、音视频、压缩包、3D 资源</h3><p>Git 能记录这些文件，但默认很难像文本一样展示内部差异、自动合并。</p></div>
</div>

<div class="callout text-red mt-5"><h3>文件很大，历史也会变得沉重</h3><p>反复提交大型资源，会增加仓库存储和传输负担；只删除当前文件，不会清掉历史里的旧版本。</p></div>

<p class="small muted">游戏项目中的代码仍适合 Git；大量二进制美术资源，往往需要额外的管理方案。部分格式也能借助专用工具比较。</p>

---

## 按问题选择补充工具或替代方案

<div class="card-grid three">
  <div class="card text-blue"><h3>大文件：Git LFS</h3><p>Git 中保留指针，大文件内容交给独立存储。</p><p class="small mt-3">改善大文件管理，不会让二进制内容自动变得可合并。</p></div>
  <div class="card text-peach"><h3>难合并：资源锁定</h3><p>编辑前先锁定，减少多人同时改同一资源。</p><p class="small mt-3">例如 Git LFS 的锁定功能，需要服务端与团队流程配合。</p></div>
  <div class="card text-green"><h3>其他方案</h3>
    <ul>
      <li>Perforce P4：集中式版本控制，适合大型二进制资源。</li>
      <li>Lore：Epic Games 推出的下一代开源 VCS。</li>
      <li>SVN、Mercurial 等其他版本控制系统。</li>
    </ul>
  </div>
</div>

<p class="source-note">参考：
  <a href="https://git-lfs.com/">Git LFS</a> · 
  <a href="https://github.com/git-lfs/git-lfs/blob/main/docs/api/locking.md">LFS 文件锁定</a> · 
  <a href="https://help.perforce.com/helix-core/quickstart/current/Content/quickstart/overview-of-helix-core.html">Perforce P4</a> · 
  <a href="https://lore.org/">Lore</a>
</p>

---
layout: section
---

# <Counter :level="1" /> 编辑器集成

<p>开发工具链 + 编辑器集成 = IDE</p>

<!--
本章 9 分钟｜累计 01:48–01:57
静态示例，无实机演示。
-->

---
layout: two-cols
---

```text
import { onMounted, ref } from 'vue'

// Reactive state.
const count = ref(0)

function increment() {
  count.value++
}

onMounted(() => {
  console.log(`The initial count is ${count.value}.`)
})
```

```ts twoslash
import { onMounted, ref } from "vue";

// Reactive state.
const count = ref(0);

function increment() {
  count.value++;
}

onMounted(() => {
  console.log(`The initial count is ${count.value}.`);
});
```

::right::

<br>
<br>
<br>
<br>
<br>
<br>

<h3 class="text-center">IDE / 编辑器如何给你的代码上色？</h3>

---

# <Counter /> 语言服务：让编辑器理解代码语法

<div class="flow">
  <div class="diagram-box text-blue">VS Code<br>显示代码与提示<br>接收用户操作</div>
  <div class="flow-arrow text-center">⇄<br>扩展 API</div>
  <div class="diagram-box text-peach">Pylance 扩展的客户端<br>接入编辑器<br>转发请求与结果</div>
  <div class="flow-arrow text-center">⇄<br>语言服务通信</div>
  <div class="diagram-box text-green"><div>Pylance 语言服务器<br>分析代码、返回提示<hr>内部使用 Pyright<br>类型分析引擎</div></div>
</div>

<div class="two-col">
  <div class="card text-blue"><h3>编辑器 → 服务</h3><p>文件内容变了；光标在这里，需要补全、悬停说明或定义位置。</p></div>
  <div class="card text-green"><h3>服务 → 编辑器</h3><p>返回候选项、类型信息、位置；发布代码诊断，由界面展示。</p></div>
</div>

<p class="small">LSP（Language Server Protocol）约定这类通信，让分析能力可以接入不同编辑器。</p>

<!--
这是职责示意，不是进程数量或部署拓扑。Pylance 扩展提供客户端接入和语言服务器，内部使用 Pyright 的分析能力；不要讲成额外启动一个独立 Pyright 服务器。
Pyright 本身也有独立语言服务器，但不是本图的部署方式。通用 LSP 架构不代表 Pylance 可在任意编辑器使用。
资料：https://github.com/microsoft/pylance-release
资料：https://code.visualstudio.com/api/language-extensions/overview
资料：https://code.visualstudio.com/api/language-extensions/language-server-extension-guide
-->

---

## 一个例子

```ts twoslash
// @errors: 2322
// Returns a greeting for the given name.
function greet(name: string): string {
  return "Hello, " + name;
}

const message = greet(42);

console.log(mes);
//             ^|
```

<div class="card-grid five">
  <div class="card text-sky"><h4>语法高亮</h4></div>
  <div class="card text-peach"><h4>自动补全</h4></div>
  <div class="card text-blue"><h4>悬停信息</h4></div>
  <div class="card text-green"><h4>定义跳转</h4></div>
  <div class="card text-red"><h4>错误诊断</h4></div>
</div>

<p class="lead">这些信息来自代码静态分析，而不是实际运行结果。</p>

---

# <Counter /> 检查与格式化：工具结果进入编辑器

<div class="card-grid three">
  <div class="card text-blue"><h3>类型检查</h3><p>例如 Pylance / Pyright<br>把类型问题定位到代码。</p></div>
  <div class="card text-peach"><h3>Lint</h3><p>例如 Ruff<br>提示未使用的导入等问题，部分问题可快速修复。</p></div>
  <div class="card text-green"><h3>格式化</h3><p>例如 Ruff formatter / Black<br>触发格式化后，编辑器应用工具返回的文本修改。</p></div>
</div>

<div class="callout text-pink mt-5"><h3>界面相似，来源可能不同</h3><p>红线和 Problems 列表可以汇集多个工具的诊断。先看来源，再理解提示；并非所有功能都经过语言服务器。</p></div>

<p class="small">前章的独立工具仍然成立：编辑器中的结果会受工具版本、配置及项目环境影响。</p>

<!--
不重复前章工具定义与命令行用法。快速修复只覆盖工具支持的修改，应用后仍需检查。
资料：https://code.visualstudio.com/docs/python/linting
资料：https://code.visualstudio.com/docs/python/formatting
-->

---

# <Counter /> 测试集成：从测试文件到结果树

<div class="flow">
  <div class="diagram-box text-blue">VS Code 测试界面</div><div class="flow-arrow">⇄</div>
  <div class="diagram-box text-peach">Python 扩展<br>测试集成</div><div class="flow-arrow">⇄</div>
  <div class="diagram-box text-green">pytest / unittest<br>收集并执行测试</div>
</div>

<div class="two-col">
  <div>

```text
测试结果（示意）
tests/test_greet.py
  ✓ test_greet_name
  ✗ test_greet_empty
    期望："Hello, guest"
    实际："Hello, "
```

  </div>
  <div class="card text-peach"><h3>同一套测试，多种入口</h3><p>发现 / 收集：形成测试列表。<br>执行：运行单个测试或一组测试。<br>展示：状态、输出与失败位置。</p></div>
</div>

<!--
pytest 的 collection 由框架按规则确定测试项，扩展将其接入测试 UI。测试需要在正确环境中执行。
资料：https://code.visualstudio.com/docs/python/testing
-->

---

# <Counter /> Git 集成：展示和应用 diff

```diff
 def greet(name: str) -> str:
-    return "Hello, " + name
+    return "Hello, " + (name or "guest")
```

<div class="card-grid three">
  <div class="card text-blue"><h3>编辑时</h3><p>行号旁显示增删改标记，打开 diff 查看具体变化。</p></div>
  <div class="card text-peach"><h3>提交前</h3><p>查看文件或局部改动，选择暂存范围，再提交。</p></div>
  <div class="card text-green"><h3>合并时</h3><p>显示冲突双方的内容，辅助选择或编辑最终结果。</p></div>
</div>

<p class="lead">VS Code 的 Git 集成提供了 GUI 界面，但底层仍然是在调用 Git</p>

<!--
静态 diff 是修复空名字测试的示意。行内装饰不是提交记录，也不保证显示任意未保存编辑的 Git 状态。
资料：https://code.visualstudio.com/docs/sourcecontrol/overview
-->

---

# <Counter /> 调试器集成

<div class="two-col">
  <div>

```python {2}
def greet(name: str) -> str:
    return "Hello, " + name  # 在此设断点

message = greet("RM")
```

<p class="small">暂停在第 2 行执行前：<br>变量：<code>name = "RM"</code><br>调用栈：<code>greet → 模块顶层</code></p>

  </div>
  <div class="card text-peach"><h3>观察正在运行的程序</h3><p>断点：执行到这里时暂停。<br>单步：逐步观察执行过程。<br>变量：查看当前值。<br>调用栈：查看如何调用到这里。</p></div>
</div>

<div class="flow">
  <div class="diagram-box text-blue">VS Code 调试界面</div><div class="flow-arrow">⇄</div>
  <div class="diagram-box text-peach">Python Debugger 扩展<br>debugpy 调试适配与后端</div><div class="flow-arrow">⇄</div>
  <div class="diagram-box text-green">运行中的 Python 程序</div>
</div>

<!--
与语言服务器的静态分析区分：这里观察的是一次实际执行。图中合并调试适配器与后端，不展开 DAP 与进程细节。
资料：https://code.visualstudio.com/docs/python/debugging
-->

---

# <Counter :level="1" /> 总结

<div class="card-grid">
  <div class="card text-blue"><h3>程序的运行环境</h3><p>在哪个系统、哪个目录下运行？调用的是哪个程序？依赖装在哪里？一个完整的项目除了代码，也包括环境要求。</p></div>
  <div class="card text-peach"><h3>检查代码，定位问题</h3><p>用类型检查和 Lint 发现代码中的问题，用测试检查结果是否符合预期；出错时读报错、设断点、看变量，修改后再运行测试确认。</p></div>
  <div class="card text-green"><h3>用 Git 记录和协作</h3><p>提交前看 diff，确认改了什么、是否正确；把相关改动组织成一次提交，写清修改原因，方便自己回顾和队友理解。</p></div>
  <div class="card text-pink"><h3>配置顺手的开发环境</h3><p>给编辑器接入项目的工具链，让补全、检查、格式化、测试和调试都能在编辑器里完成。现代化的开发体验来自这些工具的配合。</p></div>
</div>

<!--
总结与结束 3 分钟｜累计 01:57–02:00
-->

---
layout: end
---

Thank you
