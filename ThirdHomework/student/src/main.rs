

// 题目：请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作。

use mysql::*;
use mysql::prelude::*;

// 定义学生结构体
#[derive(Debug, PartialEq,Eq)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    class_id: u32,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     // 连接到MySQL数据库
     let url: &str = "mysql://root:root@localhost:3306/test_local";
     let pool = Pool::new(url)?;
     let mut conn = pool.get_conn()?;
 
     // 创建学生表
     conn.query_drop(
         r"CREATE TABLE IF NOT EXISTS students (
             id INT PRIMARY KEY AUTO_INCREMENT,
             name VARCHAR(100) NOT NULL,
             age TINYINT,
             class_id INT
         )",
     )?;


     // 添加学生
    let student1 = Student {
        id: 1,
        name: "Alice".to_string(),
        age: 18,
        class_id: 1,
    };
    insert_student(&mut conn, &student1).await?;

    let student2 = Student {
        id: 2,
        name: "Bob".to_string(),
        age: 19,
        class_id: 2,
    };
    insert_student(&mut conn, &student2).await?;

    // 查询所有学生
    let students = query_students(&mut conn).await?;
    for student in students {
        println!("Student: {} (ID: {}), Age: {}, Class ID: {}", student.name, student.id, student.age, student.class_id);
    }

    // 更新学生信息
    update_student(&mut conn, 1, "Alice Smith".to_string(), 20, 1).await?;

    // 删除学生
    delete_student(&mut conn, 2).await?;

     Ok(())
}


// 添加学生
async fn insert_student(conn: &mut PooledConn, student: &Student) -> Result<()> {
    conn.exec_drop(
        r"INSERT INTO students (id, name, age, class_id) VALUES (:id, :name, :age, :class_id)",
        params! {
            "id" => student.id,
            "name" => &student.name,
            "age" => student.age,
            "class_id" => student.class_id,
        },
    )?;
    Ok(())
}


// 查询所有学生
async fn query_students(conn: &mut PooledConn) -> Result<Vec<Student>> {
    let rows = conn.query(
        "SELECT id, name, age, class_id FROM students",
    )?;

    let students: Vec<Student> = rows.into_iter().map(|row| {
        let (id, name, age, class_id) = mysql::from_row(row);
        Student { id, name, age, class_id }
    }).collect();

    Ok(students)
}

// 更新学生信息
async fn update_student(conn: &mut PooledConn, student_id: u32, new_name: String, new_age: u8, new_class_id: u32) -> Result<()> {
    conn.exec_drop(
        r"UPDATE students SET name = :name, age = :age, class_id = :class_id WHERE id = :id",
        params! {
            "id" => student_id,
            "name" => new_name,
            "age" => new_age,
            "class_id" => new_class_id,
        },
    )?;

    Ok(())
}


// 删除学生
async fn delete_student(conn: &mut PooledConn, student_id: u32) -> Result<()> {
    conn.exec_drop(
        r"DELETE FROM students WHERE id = :id",
        params! {
            "id" => student_id,
        },
    )?;

    Ok(())
}