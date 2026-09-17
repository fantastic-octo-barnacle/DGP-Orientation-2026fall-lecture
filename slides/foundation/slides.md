---
theme: '@ktym4a/slidev-theme-ktym4a'
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
layout: section
---

# <Counter :level="1" /> 操作系统与 WSL

<p>程序工作的地基</p>

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

---

## <CounterDisplay /> WSL

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

```powershell
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
PS> Get-Location                      # 显示当前目录
PS> Get-ChildItem                     # 列出当前目录的文件和子目录
PS> Set-Location projects/homework1   # 切换到指定目录（相对路径）
PS> Get-Location
PS> Set-Location ..                   # 切换到上一级目录
PS> Get-Location

PS> Set-Location ~/courses/ECE3080    # （绝对路径）
PS> Get-Location
PS> Get-ChildItem ~                   # 列出家目录的文件和子目录
PS> Get-ChildItem -Force ~            # 包括隐藏文件
```

  </div>
  <div>
    <div class="terminal-label text-green">macOS / Linux · Bash</div>

```bash
$ pwd                    # print working directory
$ ls                     # list files and directories
$ cd projects/homework1  # change directory
$ pwd
$ cd ..
$ pwd

$ cd ~/courses/ECE3080
$ pwd
$ ls ~                   # list files and directories in home
$ ls -a ~                # list all files, including hidden ones
```

  </div>
</div>

许多 Bash 命令十分经典，PowerShell 也常常提供别名来兼容 Bash 的命令名称（但参数和含义不一定完全等价）。

---
layout: section
---

# <Counter :level="1" /> 环境变量，PATH

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
    <div class="terminal-label text-blue">PowerShell</div>

```powershell
Get-Command git
where.exe git
```
  </div>
  <div>
    <div class="terminal-label text-green">macOS / Linux</div>

```bash
command -v git
where git
which git
```
  </div>
</div>

`PATH` 是有序的，Shell 会按顺序查找每个目录，直到找到第一个可执行文件。

如果有多个同名程序，Shell 只会执行第一个找到的。

---
layout: section
---

# <Counter :level="1" /> 代码如何运行

<p>源码到行为的不同路径</p>

---

# <Counter /> CPU 执行机器指令

<br>
<br>
<br>

<div class="flow">
  <div class="diagram-box text-pink">纯文本的源代码</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">解释器、运行时<br>或编译器</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">二进制机器指令</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-blue">CPU 执行</div>
</div>

<p class="muted">不同语言的区别之一，是源码到机器执行之间经过了哪些工具和中间结果。</p>

---

## 脚本语言/解释型语言：由运行时读取

<div class="stack">
  <div class="diagram-box text-pink">Python / JavaScript 源码</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-mauve">CPython / 浏览器 / Node.js</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-peach">运行时读取、转换并执行</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-green">进程产生程序行为</div>
</div>

<p class="small muted">脚本源码不必先变成一个独立的本机可执行文件。</p>

---

## 编译型语言：先编译，再启动

<div class="stack">
  <div class="diagram-box text-pink">C / C++ / Rust 源码</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-mauve">编译与必要的链接</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-peach">本机可执行文件</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-green">操作系统启动进程（仍然可能需要相应的环境，比如动态链接库等）</div>
</div>

<p class="small muted">编译工具链、目标系统和链接所需的库都会影响构建结果。</p>

---

# <Counter /> 四种语言：比较路径，不比较优劣

<table class="compact">
  <thead>
    <tr><th>源码</th><th>中间角色</th><th>直接运行的对象</th></tr>
  </thead>
  <tbody>
    <tr><td>Python</td><td>CPython</td><td>解释器进程</td></tr>
    <tr><td>JavaScript</td><td>浏览器或 Node.js</td><td>运行时进程</td></tr>
    <tr><td>C</td><td>编译器与链接器</td><td>本机可执行文件</td></tr>
    <tr><td>Rust</td><td>Cargo 调用工具链</td><td>本机可执行文件</td></tr>
  </tbody>
</table>

<p class="muted">课堂不解释语法，不比较性能，也不展开字节码、JIT、LLVM、ABI 或链接器内部。</p>

---

# <Counter /> 同一份源码，不保证得到同一个文件

<div class="two-col">
  <div class="stack">
    <div class="diagram-box text-pink">源码相同</div>
    <div class="diagram-box text-peach">工具链版本不同</div>
    <div class="diagram-box text-red">目标系统不同</div>
  </div>
  <div class="stack">
    <div class="diagram-box text-green">构建结果可能不同</div>
    <div class="diagram-box text-blue">链接库可能不同</div>
    <div class="diagram-box text-mauve">行为仍需验证</div>
  </div>
</div>

---

# <Counter /> 编译成功，只说明一个问题

<div class="big-word text-green">能构建</div>

<div class="card-grid">
  <div class="card text-peach"><h3>不自动说明</h3><p>业务逻辑正确。</p></div>
  <div class="card text-red"><h3>不自动说明</h3><p>输入边界被处理。</p></div>
</div>

<p class="lead">“能否产生程序”与“程序是否符合预期”是两类问题。</p>

---
layout: section
---

# <Counter :level="1" /> 包、版本、依赖与开发环境

<p>82–94 分钟 · 一个项目还需要什么？</p>

---

# <Counter /> “开发环境”不是一个目录

<div class="stack">
  <div class="diagram-box text-pink">操作系统与硬件架构</div>
  <div class="diagram-box text-mauve">解释器、运行时或编译工具链及版本</div>
  <div class="diagram-box text-red">系统库与链接环境</div>
  <div class="diagram-box text-peach">第三方包及其版本</div>
  <div class="diagram-box text-green">项目源码、配置和环境变量</div>
</div>

<p class="small muted">虚拟环境只解决其中一部分隔离问题，不等同于虚拟机。</p>

---

# <Counter /> 包名写进源码，不会自动出现

<div class="flow">
  <div class="diagram-box text-pink">源码中引用包名</div>
  <div class="flow-arrow">≠</div>
  <div class="diagram-box text-red">包已经安装</div>
  <div class="flow-arrow">需要</div>
  <div class="diagram-box text-green">取得并安装到正确环境</div>
</div>

<div class="callout text-peach">
  <p>编辑器能补全一个名称，也不代表解释器或编译器真的能找到对应包。</p>
</div>

---

# <Counter /> 版本是行为的一部分

<div class="two-col">
  <div class="card text-blue">
    <h3>项目要求</h3>
    <p><code>package &gt;= 2.0, &lt; 3.0</code></p>
    <p class="small mt-3">描述可接受的范围。</p>
  </div>
  <div class="card text-red">
    <h3>实际选择</h3>
    <p><code>package 2.4.1</code></p>
    <p class="small mt-3">包管理器解析出的具体版本。</p>
  </div>
</div>

<p class="lead">“已经安装”不等于“版本匹配”。</p>

---

# <Counter /> 直接依赖会带来间接依赖

<div class="flow">
  <div class="diagram-box text-pink">项目</div>
  <div class="flow-arrow">使用</div>
  <div class="diagram-box text-peach">直接依赖 A</div>
  <div class="flow-arrow">继续使用</div>
  <div class="diagram-box text-green">间接依赖 B</div>
</div>

<div class="card-grid">
  <div class="card text-mauve"><h3>构建依赖</h3><p>构建项目时需要，运行程序时未必仍然需要。</p></div>
  <div class="card text-blue"><h3>运行依赖</h3><p>程序启动或工作时仍然需要。</p></div>
</div>

---

# <Counter /> 清单、锁文件与可重建内容

<table class="compact">
  <thead><tr><th>概念</th><th>Python 示例</th><th>Rust 示例</th></tr></thead>
  <tbody>
    <tr><td>项目清单</td><td><code>pyproject.toml</code></td><td><code>Cargo.toml</code></td></tr>
    <tr><td>锁文件</td><td><code>uv.lock</code></td><td><code>Cargo.lock</code></td></tr>
    <tr><td>本机环境 / 构建结果</td><td><code>.venv</code></td><td><code>target/</code></td></tr>
  </tbody>
</table>

<p class="muted">清单描述要求；锁文件记录具体解析结果；本机生成内容应当能够重建。</p>

---

# <Counter /> 一个更可靠的项目边界

<div class="flow">
  <div class="diagram-box text-pink">源码</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-peach">清单与锁文件</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-blue">工具链约定</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">可重建的环境</div>
</div>

<p class="lead text-center">项目不是“我电脑上的那个目录”，而是一组可说明、可恢复的条件。</p>

---
layout: section
---

# <Counter :level="1" /> Git、GitHub 与项目基本结构

<p>94–106 分钟 · 怎样记录项目及其变化？</p>

---

# <Counter /> Git 与 GitHub 是两个对象

<div class="two-col">
  <div class="card text-pink">
    <h3>Git</h3>
    <p>记录文件版本和变化历史的工具，可以只在本地使用。</p>
  </div>
  <div class="card text-blue">
    <h3>GitHub</h3>
    <p>托管 Git 仓库并支持协作的平台，不等于 Git 本身。</p>
  </div>
</div>

<p class="lead">本地 commit 不会自动出现在 GitHub；push 才会发送相应提交。</p>

---

# <Counter /> 从修改到提交，再到远程

<div class="flow">
  <div class="diagram-box text-pink">工作区<br><span class="small">文件正在变化</span></div>
  <div class="flow-arrow">add</div>
  <div class="diagram-box text-peach">暂存区<br><span class="small">选择本次变化</span></div>
  <div class="flow-arrow">commit</div>
  <div class="diagram-box text-green">本地历史</div>
  <div class="flow-arrow">push</div>
  <div class="diagram-box text-blue">远程仓库</div>
</div>

<p class="small muted">pull 则是把远程已有的变化带回本地工作流程；clone 还会取得仓库历史和远程信息。</p>

---

# <Counter /> 四个命令，观察四种状态

<div class="card-grid">
  <div class="card text-pink"><h3><code>git status</code></h3><p>现在有什么变化？</p></div>
  <div class="card text-peach"><h3><code>git diff</code></h3><p>具体改了什么？</p></div>
  <div class="card text-green"><h3><code>git log</code></h3><p>已经记录过什么？</p></div>
  <div class="card text-blue"><h3><code>git push</code></h3><p>哪些本地提交要发送到远程？</p></div>
</div>

---

# <Counter /> 一次提交应该能说明一个变化

<div class="two-col">
  <div class="callout text-green">
    <h3>好的描述</h3>
    <p>“补充路径示例”<br>“修正错误分类”</p>
  </div>
  <div class="callout text-red">
    <h3>需要警惕</h3>
    <p>把很多互不相关的改动塞进一个提交，只留下“保存一下”。</p>
  </div>
</div>

<p class="muted">提交不是上传，也不是自动备份；它首先是本地历史中的一个节点。</p>

---

# <Counter /> 一个项目目录里有什么

<div class="card-grid three">
  <div class="card text-pink"><h3>长期维护</h3><p>源码、<code>README.md</code>、测试。</p></div>
  <div class="card text-peach"><h3>环境描述</h3><p>项目清单、锁文件、工具配置。</p></div>
  <div class="card text-blue"><h3>本机生成</h3><p><code>.venv/</code>、<code>target/</code> 等可重建内容。</p></div>
</div>

<div class="callout text-red mt-4">
  <p><code>.gitignore</code> 只影响尚未被跟踪的文件；它不会把已经提交的文件从历史中删除。</p>
</div>

---

# <Counter /> 随讲随做：观察一次纯文本修改

<div class="terminal-single">

```text
$ git status
$ git diff
$ git add README.md
$ git commit -m "补充说明"
$ git log --oneline -1
```
</div>

<div class="activity-box text-sky mt-4">
  <h3>课堂安排</h3>
  <p>候选人先修改并观察 status / diff；讲师演示 add、commit 和 log。账号、权限或身份配置不在课堂集中排障。</p>
</div>

---
layout: section
---

# <Counter :level="1" /> 测试与开发检查工具

<p>106–112 分钟 · 不同工具分别回答什么问题？</p>

---

# <Counter /> 四类检查，不是四个同义词

<table class="compact">
  <thead><tr><th>检查</th><th>主要问题</th><th>典型例子</th></tr></thead>
  <tbody>
    <tr><td>格式化</td><td>排版是否统一？</td><td>Ruff formatter / cargo fmt</td></tr>
    <tr><td>静态检查</td><td>是否存在约定禁止的问题？</td><td>Ruff check / cargo clippy</td></tr>
    <tr><td>类型检查</td><td>类型使用是否一致？</td><td>Pyright / Rust 编译器</td></tr>
    <tr><td>测试</td><td>实际行为是否符合断言？</td><td>pytest / cargo test</td></tr>
  </tbody>
</table>

---

# <Counter /> “能运行”仍然可能有问题

<div class="flow">
  <div class="diagram-box text-green">能运行</div>
  <div class="flow-arrow">≠</div>
  <div class="diagram-box text-peach">排版统一</div>
  <div class="flow-arrow">≠</div>
  <div class="diagram-box text-blue">类型一致</div>
  <div class="flow-arrow">≠</div>
  <div class="diagram-box text-red">行为正确</div>
</div>

<p class="lead">一次检查通过，不能替代其他检查，也不能证明程序没有缺陷。</p>

---

# <Counter /> CI 把已有检查放到新环境执行

<div class="flow">
  <div class="diagram-box text-pink">本地运行检查</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">写入项目约定</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">CI 新环境自动执行</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-blue">反馈结果</div>
</div>

<div class="callout text-red">
  <p>CI 不是新的代码质量标准；配置里没有写的检查，它不会自动替你完成。</p>
</div>

---

# <Counter /> 本地与 CI 的关系

<div class="two-col">
  <div class="card text-green"><h3>本地检查</h3><p>反馈快，适合边改边验证。</p></div>
  <div class="card text-blue"><h3>CI 检查</h3><p>环境新、步骤固定，能暴露“我电脑上能跑”的差异。</p></div>
</div>

<p class="muted">正式招新项目中的 CI 是扩展任务，不是两条路线的主线门槛。</p>

---
layout: section
---

# <Counter :level="1" /> 报错、搜索、求助与 AI 验证

<p>112–118 分钟 · 失败是下一步的证据</p>

---

# <Counter /> 报错说明模型与实际状态不一致

<div class="two-col">
  <div class="big-word text-red">失败</div>
  <div class="stack">
    <div class="diagram-box text-pink">我以为程序在哪里</div>
    <div class="diagram-box text-peach">实际程序在另一个位置</div>
    <div class="diagram-box text-green">下一步：获取能区分两者的信息</div>
  </div>
</div>

<p class="lead">报错不是单纯的失败标志，而是当前模型不够准确的证据。</p>

---

# <Counter /> 先判断问题属于哪一层

<div class="card-grid three">
  <div class="card text-pink"><h3>命令 / PATH</h3><p>找不到程序、命中错误版本。</p></div>
  <div class="card text-peach"><h3>文件 / 路径</h3><p>当前目录不对、相对路径解析错误。</p></div>
  <div class="card text-mauve"><h3>工具 / 系统</h3><p>解释器、编译器未安装或版本不匹配。</p></div>
  <div class="card text-blue"><h3>包 / 依赖</h3><p>缺失、解析失败或构建失败。</p></div>
  <div class="card text-red"><h3>源码</h3><p>语法、类型或编译错误。</p></div>
  <div class="card text-green"><h3>运行 / 行为</h3><p>异常、错误输出或测试失败。</p></div>
</div>

---

# <Counter /> 排障是一个循环

<div class="timeline">
  <div class="text-pink"><strong>1</strong><span>观察完整现象</span></div>
  <div class="text-mauve"><strong>2</strong><span>判断问题层次</span></div>
  <div class="text-peach"><strong>3</strong><span>提出一个猜测</span></div>
  <div class="text-blue"><strong>4</strong><span>查文档、搜索或求助</span></div>
  <div class="text-green"><strong>5</strong><span>做最小实验并复查</span></div>
</div>

<p class="lead text-center mt-5">一次只改变一个关键变量，结果才有解释空间。</p>

---

# <Counter /> 一个有用的问题包含上下文

<div class="check-list">
  <div><span class="check-icon text-pink">01</span><span>想完成什么？</span></div>
  <div><span class="check-icon text-peach">02</span><span>实际执行了什么命令或操作？</span></div>
  <div><span class="check-icon text-red">03</span><span>完整错误信息是什么？</span></div>
  <div><span class="check-icon text-blue">04</span><span>操作系统、工具和关键版本是什么？</span></div>
  <div><span class="check-icon text-green">05</span><span>已经尝试过什么，怎样稳定复现？</span></div>
</div>

---

# <Counter /> 搜索与求助：保留稳定关键词

<div class="two-col">
  <div class="card text-green">
    <h3>保留</h3>
    <p>错误类型、稳定的错误短语、项目名、工具名和版本。</p>
  </div>
  <div class="card text-red">
    <h3>去除</h3>
    <p>只属于本机的用户名、绝对路径和无关滚屏。</p>
  </div>
</div>

<p class="lead">优先看官方文档、项目文档，以及与当前版本匹配的信息。</p>

---

# <Counter /> 使用 AI 前后都要验证

<div class="card-grid">
  <div class="card text-red"><h3>不要提供</h3><p>密码、令牌、私有密钥或不应公开的项目内容。</p></div>
  <div class="card text-peach"><h3>执行前检查</h3><p>命令会读取、修改还是删除什么？适用于哪个系统和 Shell？</p></div>
  <div class="card text-green"><h3>结论要复核</h3><p>用官方文档、实际运行、测试或最小实验验证。</p></div>
  <div class="card text-blue"><h3>最终要能解释</h3><p>能运行不等于理解正确；保留的方案需要说得清楚。</p></div>
</div>

---

# <Counter /> 随讲随答：先决定下一条信息

<div class="activity-box text-sky">
  <h3>面对一个失败现象，快速回答：</h3>
  <ol>
    <li>它最可能属于哪一层？</li>
    <li>还缺少哪条信息？</li>
    <li>搜索时保留哪些关键词？</li>
    <li>如果 AI 给出删除或修改环境的命令，执行前检查什么？</li>
  </ol>
</div>

<p class="muted">不要求现场解决全新的复杂故障，先学会把问题变得可判断。</p>

---
layout: section
---

# <Counter :level="1" /> 路线选择与正式项目交接

<p>118–120 分钟 · 从共同语言走向自主学习</p>

---

# <Counter /> 两条路线是起点，不是能力等级

<div class="two-col">
  <div class="card route-card text-pink">
    <strong>Python 起点</strong>
    <ul>
      <li>尚不能独立编写并运行简单程序。</li>
      <li>先补齐编程与开发环境基础。</li>
    </ul>
  </div>
  <div class="card route-card text-blue">
    <strong>Rust 起点</strong>
    <ul>
      <li>已经理解变量、分支、循环和函数。</li>
      <li>能用任意语言完成简单输入、处理和输出。</li>
      <li>不要求此前学过 Rust。</li>
    </ul>
  </div>
</div>

<p class="lead text-center">路线可以调整，不登记，也不代表录取优先级。</p>

---

# <Counter /> 用自测决定从哪里开始

<div class="check-list">
  <div><span class="check-icon text-pink">A</span><span>我能否独立创建、运行并修改一个简单程序？</span></div>
  <div><span class="check-icon text-peach">B</span><span>我是否理解变量、分支、循环和函数？</span></div>
  <div><span class="check-icon text-green">C</span><span>我能否说明输入、处理和输出分别发生了什么？</span></div>
</div>

<p class="source-note"><a href="../../training/foundation/route-self-check.md">打开路线选择自测</a></p>

---

# <Counter /> 正式项目看重可说明的学习证据

<div class="flow">
  <div class="diagram-box text-pink">环境</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-peach">代码与文档</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-blue">运行与检查</div>
  <div class="flow-arrow">+</div>
  <div class="diagram-box text-green">问题与迭代</div>
</div>

<div class="callout text-mauve">
  <p>可以查资料、使用 AI、讨论和求助；最终需要理解并解释自己的环境、代码和决策。</p>
</div>

---

# <Counter /> 接下来怎么走

<div class="card-grid three">
  <div class="card text-pink"><h3>1</h3><p>完成或跳过已经掌握的预备内容。</p></div>
  <div class="card text-peach"><h3>2</h3><p>阅读两条路线的起始任务。</p></div>
  <div class="card text-green"><h3>3</h3><p>选择合适起点，留下真实、可运行、可说明的成果。</p></div>
</div>

<div class="callout text-blue mt-4">
  <p>正式项目入口将在项目 spec 定稿后补充；先从路线自测和起始任务开始。</p>
</div>

---
layout: statement
---

# 不必一次记住所有命令

先观察现象，再判断边界；做一个小实验，验证一个猜测。

<p class="mt-6 muted">计算机开发基础 · 课程结束</p>
