//变量绑定 let a1 = 5;  类似c++ 常量  不可变   虽然变量不可变但是变量可以重新let使用 重影
//可变绑定 let mut=10 ;


//内置数据类型
bool
char 字符类型 u8可以转为char  4个字节 可支持中文  所以在 Rust 中字符串和字符都必须使用 UTF-8 编码，否则编译器会报错。
number  i8  i16 i32 i64 i128 isize 默认i32
        i8  u16 u32 u64 u128 usize
        f32 f64  默认f32
str  字符串类型：最底层的是不定长类型str，更常用的是字符串切片&str和堆分配字符串String， 其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。
数组：具有固定大小，并且元素都是同种类型，可表示为[T; N]。
切片：引用一个数组的部分数据并且不需要拷贝，可表示为&[T]。
元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
指针：最底层的是裸指针*const T和*mut T，但解引用它们是不安全的，必须放到unsafe块里。
函数：具有函数类型的变量实质上是一个函数指针。
元类型：即()，其唯一的值也是()。
不同类型不支持运算  使用as 转型
元组 使用()包括的一组数据 可包含不同类型
数组 使用[]包括的同类型数据  a.1  a.2获取数据

注释 // /*   */   ///文档注释
函数 fn fname( param) ->rettype {}

条件 if a>0{} else if a<5 {} else{} 条件必须为bool类型

循环 while a<10{  a+=1;}
let a=[1,4,55,56] 
for i in a.iter(){println!("{}",i)}  ///打印1,4,55,56
for (index, value) in a.iter().enumerate(){ //index是下标，value是值
        println!("index = {}, value = {}", index, value);
 }
for i in 0..5{println!("{}",i); } /// 打印0,1,2,3,4

    iter() 创造一个不可修改值的循环器;
    iter_mut()方法；创造一个可修改值的循环器；
    元组模式，arr.iter().enumerate()；创造一个不可修改值的循环器，
    take(num)限制前num条数据  arr.iter().take(5).enumerate()

let t=0;
let a=loop {
    if t>10{break t;}
    t+=1;
}
    
切片 data[a..b]
打印 println!("{} {:?} {:#?}",a)  
返回值   函数最后一行 没有;

const不变的值  不能被重影
static   不变的值 有固定存储位置
在main外编写
const NUMBNER_OF_MONTHS:u32=12;

数组
let mut array:[i32;3]=[0;3];

二维数组
    let mut mapdata:[[bool;5];4]=[[false;5];4];
    println!("{:?}",mapdata);
    动态数组

动态数组
    let v:Vev<i32>=Vec![]

面向对象
定义数据结构
    sturct Solutions{}
    #定义函数 
    impl Solutions{
        函数中有&self 表成员函数 
        没有表示域函数 用 Solutions::func()调用
    }
    #定义特性
    trait Print()
    #实现特性
    impl Print for Solutions{

    }

类比c++智能指针
RC refrence计数器  单线程 多线程用arc
    Rc::new("Calgary began as a fort called Fort Calgary that...".to_string())
    解开 直接使用
解引用

RefCell  RefCell::new(data), 是另一种无需声明即可更改值的方法mut。就像一个，Cell但是使用引用而不是副本。
    borrow_mut()
    borrow()

RC