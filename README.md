## nds-rust-patch-template

这是一个可以通过 Rust 编写 NDS 补丁代码，并由 ARMIPS 进行导入且引用来自 Rust 代码的简易项目模板。

既不失去汇编的自由度，又能够利用 Rust 的语言能力和代码能力，非常适合用于 NDS 游戏的汉化破解。

## 基本用法

首先需要安装 `nightly` 频道的 Rust 工具链，然后准备好 ARMIPS。

在 `src` 文件夹内编写 Rust 代码，需要导出的函数名称需要以 `rustapi_` 开头并附上 `#[no_mangle]` 标记。

随后在 `asm` 文件夹内编写汇编代码，并通过 ARMIPS 的 `.importobj` 来导入 Rust 编译产物，随后可以在汇编中调用之前导入来自 Rust 代码的函数了。

模板已内附简易的范例代码，模板的其他部分也可以自由修改以符合自己的需求。
