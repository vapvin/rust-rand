fn main() {
    let width = 30;
    let height = 50;
    print!(
        "사각형의 면적: {} 제곱픽셀",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32)-> u32 {
    width * height
}