
// TODO: 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

// enum MyEnum {
//     VariantA(TypeA),
//     VariantB(TypeB),
//     VariantC(TypeC),
// }

// struct TypeA;
// struct TypeB;
// struct TypeC;

// impl TypeA {
//     fn method_a(&self) {
//         println!("TypeA: Method A");
//     }
// }

// impl TypeB {
//     fn method_b(&self) {
//         println!("TypeB: Method B");
//     }
// }

// impl TypeC {
//     fn method_c(&self) {
//         println!("TypeC: Method C");
//     }
// }

// fn main() {
//     let vec: Vec<MyEnum> = vec![
//         MyEnum::VariantA(TypeA),
//         MyEnum::VariantB(TypeB),
//         MyEnum::VariantC(TypeC),
//     ];

//     for item in vec {
//         match item {
//             MyEnum::VariantA(a) => a.method_a(),
//             MyEnum::VariantB(b) => b.method_b(),
//             MyEnum::VariantC(c) => c.method_c(),
//         }
//     }
// }


// TODO: 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

trait MyTrait {
    fn do_something(&self);
}

struct TypeA;
struct TypeB;
struct TypeC;

impl MyTrait for TypeA {
    fn do_something(&self) {
        println!("TypeA: Doing something");
    }
}

impl MyTrait for TypeB {
    fn do_something(&self) {
        println!("TypeB: Doing something");
    }
}

impl MyTrait for TypeC {
    fn do_something(&self) {
        println!("TypeC: Doing something");
    }
}

fn main() {
    let vec: Vec<Box<dyn MyTrait>> = vec![
        Box::new(TypeA),
        Box::new(TypeB),
        Box::new(TypeC),
    ];

    for item in vec {
        item.do_something();
    }
}

// TODO: 总结
// 1.枚举方法提供了更多的编译时安全性，但需要在枚举定义中明确列出所有可能的类型。
// 2.Trait Object 方法提供了更大的灵活性和动态性，但可能在运行时需要做更多的类型检查，并且无法在编译时检查是否处理了所有可能的类型。在需要动态选择方法并且不确定有哪些类型时，Trait Object 方法是更好的选择。如果所有类型在编译时都是已知的且不会改变，那么枚举方法可能更合适。