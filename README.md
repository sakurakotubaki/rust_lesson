# Udemy Rust Lesson
Basic Rust Video

[Data Type](https://doc.rust-lang.org/book/ch03-02-data-types.html)

## 変数宣言 let
letで書く。
変数に値を紐付けるのを束縛するバインドするという。

## コメント方法
**１行**

// 2個

```rust
fn main() {
    // logを出力するときは、!が必要
    println!("Hello, world!");
}
```

**複数行**

/**/

```rust
fn main() {
    // logを出力するときは、!が必要
    /*
    println!("Hello, world!");
    print!("Hello, world!");
    */
}
```

## 定数 const
定数では大文字で書く。
スコープの外で書く。

```rust

const B: i32 = 1;
fn main() {
   const A: i32 = 1;
}
```

## タプル型
複数の値をまとめる。

```rust
fn main() {
   let t1: (i32, bool, f64) = (1, true, 2.0);
    lett2: (f64, i32, bool) = (2.0, 1, true);
    
    prin("{:?}",t1);
    
    let i = t1.0;
    
    let (x: 64, y: i32, _) = t2;
    
    let u: () = ();
}
```

## array
要素を複数扱う。

```rust
fn main() {
   // array
    let l1: [i32; 3]  = [1, 2, 3];
    let l2: [i32; 1000] = [0; 1000];

    println!("{:?}", l1);

    let i: i32 = l1[0];

    let [x,yi32,z] = l1;
    // slice
    let l3: &[i32] = &l1[0..2];
    println!("{:?}", l3);
}
```