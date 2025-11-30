use std::io;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("PAU SMSIS.txt").expect("File was not able to be created!");
    writeln!(file,"         PAU SMIS");
    writeln!(file, "Student     Matric Number    Department     level");

    let mut student_info = Vec::new();

      loop{
        //Collect Student name
        println!("Student Name: ");
        let mut student_name = String::new();
            io::stdin()
            .read_line(&mut student_name)
            .expect("Invalid Student Name");
        let student_name = student_name.trim();


        //Collect matric number
        println!("Matric Number: ");
        let mut matric_number = String::new();
            io::stdin()
            .read_line(&mut matric_number)
            .expect("Invalid Matric Number!");
        let matric_number = matric_number.trim();

        //collecting Department
        println!("Department: ");
        let mut department = String::new();
            io::stdin()
            .read_line(&mut department)
            .expect("Invalid Department");
        let department = department.trim();


        //Collecting user level
        println!("level: ");
        let mut level = String::new();
            io::stdin()
            .read_line(&mut level)
            .expect("Invalid user level");
        let level: i32 = level.trim().parse().expect("Type an integer!");
        if level % 100 != 0 {
            println!("Not a valid Student Level!");
            continue;
        }
        let levelstr: String= level.to_string();
                //storing in vector
        let v = vec![student_name.to_string(), matric_number.to_string(), department.to_string(), levelstr.to_string()];
        student_info.push(v);

        //Asking if there are more students
        println!("Is that all (Y) (N)");
        let mut is_done = String::new();
            io::stdin()
            .read_line(&mut is_done)
            .expect("Yes or No!");
        let is_done = is_done.trim();
          if is_done == "Y" {
            println!("Thank You!");
            break;
        }

        if is_done == "N" {
            continue;
        }


    }
    for info in &student_info {
        writeln!(file, "{}      {}      {}      {}",info[0],info[1], info[2], info[3])
        .expect("write failed horrendously");
    }
}