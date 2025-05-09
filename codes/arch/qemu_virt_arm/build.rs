fn main() {
    cc::Build::new()
        .file("src/asm/top.S")  // 使用 .S 文件，预处理器参与
        .compile("top");

    
}