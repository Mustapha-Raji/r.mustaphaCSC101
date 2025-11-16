use std::io;

fn main() {
    loop {
        let mut calculation_type = String::new();
        let mut selected_shape = String::new();

        println!("Do you want to calculate area or volume?");
        io::stdin().read_line(&mut calculation_type).expect("Failed to read input");
        let calculation_type = calculation_type.trim().to_lowercase();

        println!("Choose a shape:
1. trapezium
2. rhombus
3. parallelogram
4. cube
5. cylinder");
        io::stdin().read_line(&mut selected_shape).expect("Failed to read input");
        let selected_shape = selected_shape.trim().to_lowercase();

        // AREA CALCULATIONS
        if calculation_type == "area" {
            if selected_shape == "trapezium" {
                let mut height_input = String::new();
                let mut first_base_input = String::new();
                let mut second_base_input = String::new();

                println!("Enter the height of the trapezium:");
                io::stdin().read_line(&mut height_input).expect("Invalid input");
                let height: f64 = height_input.trim().parse().expect("Please enter a number");

                println!("Enter base 1:");
                io::stdin().read_line(&mut first_base_input).expect("Invalid input");
                let first_base: f64 = first_base_input.trim().parse().expect("Please enter a number");

                println!("Enter base 2:");
                io::stdin().read_line(&mut second_base_input).expect("Invalid input");
                let second_base: f64 = second_base_input.trim().parse().expect("Please enter a number");

                let area = height * (first_base + second_base) / 2.0;
                println!("The area of the trapezium is {}", area);
                break;

            } else if selected_shape == "rhombus" {
                let mut first_diagonal_input = String::new();
                let mut second_diagonal_input = String::new();

                println!("Enter diagonal 1 of the rhombus:");
                io::stdin().read_line(&mut first_diagonal_input).expect("Invalid input");
                let first_diagonal: f64 = first_diagonal_input.trim().parse().expect("Please enter a number");

                println!("Enter diagonal 2 of the rhombus:");
                io::stdin().read_line(&mut second_diagonal_input).expect("Invalid input");
                let second_diagonal: f64 = second_diagonal_input.trim().parse().expect("Please enter a number");

                let area = 0.5 * first_diagonal * second_diagonal;
                println!("The area of the rhombus is {}", area);
                break;

            } else if selected_shape == "parallelogram" {
                let mut base_input = String::new();
                let mut altitude_input = String::new();

                println!("Enter the base of the parallelogram:");
                io::stdin().read_line(&mut base_input).expect("Invalid input");
                let base: f64 = base_input.trim().parse().expect("Please enter a number");

                println!("Enter the altitude of the parallelogram:");
                io::stdin().read_line(&mut altitude_input).expect("Invalid input");
                let altitude: f64 = altitude_input.trim().parse().expect("Please enter a number");

                let area = base * altitude;
                println!("The area of the parallelogram is {}", area);
                break;

            } else if selected_shape == "cube" {
                let mut side_length_input = String::new();
                println!("Enter the length of one side of the cube:");
                io::stdin().read_line(&mut side_length_input).expect("Invalid input");
                let side_length: f64 = side_length_input.trim().parse().expect("Please enter a number");

                let surface_area = 6.0 * side_length.powf(2.0);
                println!("The surface area of the cube is {}", surface_area);
                break;

            } else if selected_shape == "cylinder" {
                println!("Sorry, surface area for a cylinder is not supported yet.");
            }
        }

        // VOLUME CALCULATIONS
        else if calculation_type == "volume" {
            if selected_shape == "cube" {
                let mut side_length_input = String::new();
                println!("Enter the length of one side of the cube:");
                io::stdin().read_line(&mut side_length_input).expect("Invalid input");
                let side_length: f64 = side_length_input.trim().parse().expect("Please enter a number");

                let volume = side_length.powf(3.0);
                println!("The volume of the cube is {}", volume);
                break;

            } else if selected_shape == "cylinder" {
                let mut radius_input = String::new();
                let mut height_input = String::new();

                println!("Enter the radius of the cylinder:");
                io::stdin().read_line(&mut radius_input).expect("Invalid input");
                let radius: f64 = radius_input.trim().parse().expect("Please enter a number");

                println!("Enter the height of the cylinder:");
                io::stdin().read_line(&mut height_input).expect("Invalid input");
                let height: f64 = height_input.trim().parse().expect("Please enter a number");

                let volume = 3.14 * radius.powf(2.0) * height;
                println!("The volume of the cylinder is {}", volume);
                break;

            } else {
                println!("Volume calculation is only available for cube and cylinder.");
            }
        }

        // INVALID INPUT
        else {
            println!("Please type 'area' or 'volume' correctly.");
        }
    }
}