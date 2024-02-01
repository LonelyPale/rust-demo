#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ReferenceStruct<'a> {
    // 这是一个指向 Point 的引用字段
    point_ref: &'a Point,
}

#[test]
fn test() {
    // std::fmt::Debug::fmt();

    let mut point = Point { x: 1, y: 2 };
    point.x *= 1;
    point.y *= 2;
    let ref_struct = ReferenceStruct { point_ref: &point };

    // 输出结构体实例的内存地址
    println!("ref_struct: {:p} {:?}", &ref_struct, ref_struct);

    // 输出引用字段的内存地址
    println!(
        "point_ref: {:p} {:?}",
        ref_struct.point_ref, ref_struct.point_ref
    );

    println!("point: {:p} {:?}", &point, point);
}
