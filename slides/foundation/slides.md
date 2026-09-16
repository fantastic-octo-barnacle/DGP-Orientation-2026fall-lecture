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
---

<style>
@import './styles/index.css';
</style>

---
layout: cover
class: cover-slide
---

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

# <CounterDisplay /> WSL

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

# <Counter :level="1" /> 终端、shell 与命令行

<p>8–18 分钟 · 谁在读取你的输入？</p>

---

# <Counter /> 终端、shell、命令行分别是什么

<div class="flow">
  <div class="diagram-box text-pink"><strong>终端</strong><br><span class="small">提供文本交互界面</span></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-mauve"><strong>shell</strong><br><span class="small">读取并解释输入</span></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach"><strong>命令</strong><br><span class="small">改变状态或启动程序</span></div>
</div>

<p class="lead">“命令行”描述的是交互形式，不是一个独立的软件名称。</p>

---

# <Counter /> shell 会记住一些状态

<div class="two-col">
  <div class="diagram-box text-red">
    <h3>当前目录</h3>
    <p>相对路径从哪里开始解释？</p>
  </div>
  <div class="diagram-box text-peach">
    <h3>环境变量</h3>
    <p>启动其他程序时，哪些名称和值会传过去？</p>
  </div>
</div>

<div class="callout text-sky mt-5">
  <p>shell 本身也是程序，但它不等于操作系统。</p>
</div>

---

# <Counter /> 有些命令改变 shell，有些命令启动程序

<div class="card-grid">
  <div class="card text-pink">
    <h3>内建命令</h3>
    <p>例如切换当前目录。它必须改变 shell 自己的状态。</p>
  </div>
  <div class="card text-green">
    <h3>外部程序</h3>
    <p>例如 Git。shell 根据规则找到磁盘上的程序并启动它。</p>
  </div>
</div>

<p class="muted">不是所有命令都对应一个同名文件，也不是所有文件都能作为命令运行。</p>

---

# <Counter /> 同一个终端窗口不是同一个会话

<div class="two-col">
  <div>
    <div class="terminal-label text-pink">会话 A</div>

```text
PS> Set-Location project
PS> Get-Location
Path
----
C:\Users\student\project
```
  </div>
  <div>
    <div class="terminal-label text-green">会话 B</div>

```text
PS> Get-Location
Path
----
C:\Users\student
```
  </div>
</div>

<p class="small muted">打开第二个终端窗口，观察它有自己的当前目录和交互状态。</p>

---

# <Counter /> 随讲随做：先只看提示符

<div class="terminal-grid">
  <div>
    <div class="terminal-label text-blue">PowerShell</div>

```text
PS> Write-Output "hello"
hello
```
  </div>
  <div>
    <div class="terminal-label text-peach">macOS / Linux</div>

```text
$ printf 'hello\n'
hello
```
  </div>
</div>

<div class="activity-box text-sky mt-4">
  <h3>观察</h3>
  <p>哪一部分像“你输入的命令”？哪一部分像“程序输出的结果”？</p>
</div>

---
layout: section
---

# <Counter :level="1" /> 文件类型与纯文本

<p>18–32 分钟 · 文件保存的是什么？</p>

---

# <Counter /> 文件系统保存的是字节

<div class="flow">
  <div class="diagram-box text-pink">磁盘上的字节</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-mauve">格式约定</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">软件解释与呈现</div>
</div>

<div class="card-grid three">
  <div class="card text-green"><h3>纯文本</h3><p>按字符编码解释的内容。</p></div>
  <div class="card text-sky"><h3>图片</h3><p>按图像格式解释像素与元数据。</p></div>
  <div class="card text-pink"><h3>复合文档</h3><p>由多个资源共同描述内容与布局。</p></div>
</div>

---

# <Counter /> 扩展名是提示，不是转换器

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

# <Counter /> 纯文本为什么适合写代码

<div class="card-grid">
  <div class="card text-green">
    <h3>保存的是字符序列</h3>
    <p>源代码、Markdown、HTML、XML 都可以是纯文本。</p>
  </div>
  <div class="card text-blue">
    <h3>编辑器只是工具</h3>
    <p>记事本、终端编辑器和 IDE 都可以修改同一份文本。</p>
  </div>
</div>

<p class="lead">IDE 能提供导航、补全和检查，但它不是源码存在的前提。</p>

---

# <Counter /> `.docx` 是一种复合文档格式

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

<p class="small muted">这是 `.docx` 的结构，不代表所有 Word 文件都相同；旧式 `.doc` 是另一种格式。</p>

---

# <Counter /> 随讲随做：创建一个纯文本文件

<div class="activity-box text-sky">
  <h3>只验证刚引入的概念</h3>
  <ol>
    <li>用普通文本编辑器新建文件。</li>
    <li>写入一句话并保存。</li>
    <li>确认文件名、扩展名和位置。</li>
    <li>重新打开，确认内容仍然是原来的文字。</li>
  </ol>
</div>

<p class="muted">不要把保存文件变成安装编辑器或排查关联程序的任务。</p>

---
layout: section
---

# <Counter :level="1" /> 文件系统、路径与导航

<p>32–47 分钟 · shell 怎样找到文件？</p>

---

# <Counter /> 文件系统是一棵目录树

<div class="two-col">
  <div class="stack">
    <div class="diagram-box text-blue">根位置</div>
    <div class="diagram-box text-mauve">用户主目录</div>
    <div class="diagram-box text-pink">当前目录</div>
  </div>
  <div class="stack">
    <div class="diagram-box text-green">子目录</div>
    <div class="diagram-box text-peach">文件</div>
    <div class="diagram-box text-red">另一个分支</div>
  </div>
</div>

<p class="lead">用户主目录是常用起点，但它不是文件系统的根。</p>

---

# <Counter /> 绝对路径与相对路径

<div class="two-col">
  <div class="card text-pink">
    <h3>绝对路径</h3>
    <p>从固定的根位置开始。无论当前目录在哪里，指向都不变。</p>
    <p class="mt-3"><code>/home/student/course/hello.txt</code></p>
  </div>
  <div class="card text-peach">
    <h3>相对路径</h3>
    <p>以当前目录为起点。同一个名字可能在不同位置指向不同文件。</p>
    <p class="mt-3"><code>./hello.txt</code></p>
  </div>
</div>

---

# <Counter /> `.`、`..` 与空格

<div class="card-grid three">
  <div class="card text-green"><h3><code>.</code></h3><p>当前目录。</p></div>
  <div class="card text-blue"><h3><code>..</code></h3><p>上一级目录。</p></div>
  <div class="card text-red"><h3>空格</h3><p>需要让 shell 知道它们属于同一个参数。</p></div>
</div>

<div class="callout text-mauve mt-5">
  <p>路径中的引号是给 shell 的边界提示，不会凭空改变文件名。</p>
</div>

---

# <Counter /> 随讲随做：每次只完成一个导航动作

<div class="terminal-grid">
  <div>
    <div class="terminal-label text-blue">PowerShell</div>

```text
PS> Get-Location
PS> Get-ChildItem
PS> Set-Location ..
PS> Get-Location
```
  </div>
  <div>
    <div class="terminal-label text-green">macOS / Linux</div>

```text
$ pwd
$ ls
$ cd ..
$ pwd
```
  </div>
</div>

<p class="small muted">课堂不发放一长串命令清单；每个命令只用来验证刚出现的概念。</p>

---

# <Counter /> 文件存在，但相对路径仍然可能失败

<div class="two-col">
  <div class="stack">
    <div class="diagram-box text-pink">文件真实位置<br><code>course/hello.txt</code></div>
    <div class="flow-arrow">↑</div>
    <div class="diagram-box text-peach">当前目录<br><code>course/scripts/</code></div>
  </div>
  <div class="callout text-red">
    <h3>现象</h3>
    <p>读取 <code>hello.txt</code> 失败。</p>
    <h3 class="mt-4">变化</h3>
    <p>切换当前目录，或改用从当前目录出发的正确路径。</p>
    <h3 class="mt-4">没有变化</h3>
    <p>文件和源码本身没有改变。</p>
  </div>
</div>

---
layout: section
---

# <Counter :level="1" /> 环境变量、PATH、可执行文件与进程

<p>47–60 分钟 · 命令名称怎样变成一次运行？</p>

---

# <Counter /> 环境变量是传给程序的名称和值

<div class="flow">
  <div class="diagram-box text-pink">shell</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach"><code>NAME=value</code></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">被启动的程序</div>
</div>

<div class="card-grid">
  <div class="card text-blue"><h3>可以表示</h3><p>路径、配置、开关、凭据等信息。</p></div>
  <div class="card text-red"><h3>不要混淆</h3><p>环境变量不是第三方依赖，也不是自动保存到项目中的配置文件。</p></div>
</div>

---

# <Counter /> PATH 是一组搜索位置

<div class="flow">
  <div class="diagram-box text-mauve"><code>git</code></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-pink">PATH 位置 1</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">PATH 位置 2</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">找到可执行程序</div>
</div>

<div class="two-col">
  <div class="callout text-red"><p>输入完整路径：明确指定启动哪个程序。</p></div>
  <div class="callout text-blue"><p>只输入名称：交给 shell 按当前环境寻找。</p></div>
</div>

---

# <Counter /> “安装了”不等于“命令能找到”

<div class="card-grid three">
  <div class="card text-pink"><h3>下载完成</h3><p>磁盘上出现了软件或文件。</p></div>
  <div class="card text-peach"><h3>位置存在</h3><p>它可能在某个目录中，但不在搜索路径里。</p></div>
  <div class="card text-green"><h3>命令可用</h3><p>当前 shell 的 PATH 能命中正确版本。</p></div>
</div>

<p class="lead">同名命令还可能因为 shell、Windows/WSL 或版本不同而命中不同位置。</p>

---

# <Counter /> 程序文件、脚本文件、数据文件

<div class="card-grid three">
  <div class="card text-red"><h3>本机可执行文件</h3><p>操作系统可以装载并执行机器指令。</p></div>
  <div class="card text-mauve"><h3>脚本</h3><p>通常需要对应解释器，启动规则也可能依赖平台。</p></div>
  <div class="card text-blue"><h3>数据文件</h3><p>通常由其他程序打开，不是文件自身“执行”。</p></div>
</div>

<p class="small muted">本课不展开 PE、Mach-O、ELF、权限位和平台命令扩展细节。</p>

---

# <Counter /> 磁盘上的程序 ≠ 运行中的进程

<div class="flow">
  <div class="diagram-box text-peach">磁盘上的<br>程序文件</div>
  <div class="flow-arrow">启动</div>
  <div class="diagram-box text-pink">进程 A</div>
  <div class="flow-arrow">再启动</div>
  <div class="diagram-box text-green">进程 B</div>
</div>

<div class="callout text-sky">
  <p>同一个可执行文件通常可以产生多个进程；是否限制单实例是应用策略，不是操作系统的一般要求。</p>
</div>

---

# <Counter /> 随讲随做：确认 Git 命中了哪里

<div class="terminal-grid">
  <div>
    <div class="terminal-label text-blue">PowerShell</div>

```text
PS> Get-Command git
PS> git --version
```
  </div>
  <div>
    <div class="terminal-label text-green">macOS / Linux</div>

```text
$ command -v git
$ git --version
```
  </div>
</div>

<div class="activity-box text-peach mt-4">
  <h3>问题</h3>
  <p>你看到的是命令名、文件路径，还是一次运行的进程？</p>
</div>

---

# 到这里，第一张大图已经闭合

<div class="flow">
  <div class="diagram-box text-pink">操作系统<br><span class="small">提供文件系统与进程</span></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-mauve">shell<br><span class="small">提供文本操作入口</span></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-red">路径<br><span class="small">定位文件</span></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">PATH<br><span class="small">寻找程序</span></div>
</div>

<p class="lead text-center">接下来：源码怎样经过工具链产生行为？</p>

---
layout: section
---

# <Counter :level="1" /> Python、JavaScript、C 与 Rust 怎样运行

<p>65–82 分钟 · 源码到行为的不同路径</p>

---

# <Counter /> CPU 最终执行机器指令

<div class="flow">
  <div class="diagram-box text-pink">人写的源码<br><span class="small">纯文本</span></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-peach">解释器、运行时<br>或编译器</div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-green">机器指令<br><span class="small">CPU 执行</span></div>
  <div class="flow-arrow">→</div>
  <div class="diagram-box text-blue">程序行为</div>
</div>

<p class="muted">不同语言的区别之一，是源码到机器执行之间经过了哪些工具和中间结果。</p>

---

# <Counter /> Python 与 JavaScript：由运行时读取

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

# <Counter /> C 与 Rust：先编译，再启动

<div class="stack">
  <div class="diagram-box text-pink">C / Rust 源码</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-mauve">编译与必要的链接</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-peach">本机可执行文件</div>
  <div class="flow-arrow">↓</div>
  <div class="diagram-box text-green">操作系统启动进程</div>
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
  <div class="card text-peach"><h3>执行前检查</h3><p>命令会读取、修改还是删除什么？适用于哪个系统和 shell？</p></div>
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
