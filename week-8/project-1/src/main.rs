use std::io;
fn office_admin() {
    let vadmin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    println!("What is your current job description?, Enter:\n0 for Intern\n1 for Administrator\n2 for Senior Administrator\n3 for Office Manager\n4 for Director\n5 for CEO");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid input");
    let role = vadmin[irole];
    println!("Your role is {}",role);
    let mut iexp = String::new();
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid input");
    if irole == 0 || iexp >= 1 && iexp <=2 {
        println!("Your APS level is: APS 1-2");
    } else if irole == 1 || iexp >= 3 && iexp <= 5 {
        println!("Your APS level is: APS 3-5");
    } else if irole == 2 || iexp >= 5 && iexp <= 8 {
        println!("Your APS level is: APS 5-8");
    } else if irole == 3 || iexp >=8 && iexp <= 10 {
        println!("Your APS level is: EL1 8-10");
    } else if irole == 4 || iexp >=10 && iexp <= 13 {
        println!("Your APS level is: EL2 10-13");
    } else if irole == 5 || iexp > 13 {
        println!("Your APS level is: SES");
    } else { println!("No APS level available");}
}
fn academic() {
    let vacademic = vec!["Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    println!("What is your current job description?, Enter:\n0 for Research Assistant\n1 for PhD Candidate\n2 for Post-Doc Researcher\n3 for Senior Lecturer\n4 for Dean");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid input");
    let role = vacademic[irole];
    println!("Your role is {}",role);
    let mut iexp = String::new();
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid input");
    if  iexp >= 1 && iexp <=2 {
        println!("Your APS level is: APS 1-2");
    } else if irole == 0 || iexp >= 3 && iexp <= 5 {
        println!("Your APS level is: APS 3-5");
    } else if irole == 1 || iexp >= 5 && iexp <= 8 {
        println!("Your APS level is: APS 5-8");
    } else if irole == 2 || iexp >=8 && iexp <= 10 {
        println!("Your APS level is: EL1 8-10");
    } else if irole == 3 || iexp >=10 && iexp <= 13 {
        println!("Your APS level is: EL2 10-13");
    } else if irole == 4 || iexp > 13 {
        println!("Your APS level is: SES");
    } else {println!("No APS level available");}
}
fn lawyer() {
    let vlawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    println!("What is your current job description?, Enter:\n0 for Paralegal\n1 for Junior Associate\n2 for Associate\n3 for Senior Associate 1-2\n4 for Senior Associate 3-4\n5 for Partner");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid input");
    let role = vlawyer[irole];
    println!("Your role is {}",role);
    let mut iexp = String::new();
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid input");
    if irole == 0 || iexp >= 1 && iexp <=2 {
        println!("Your APS level is: APS 1-2");
    } else if irole == 1 || iexp >= 3 && iexp <= 5 {
        println!("Your APS level is: APS 3-5");
    } else if irole == 2 || iexp >= 5 && iexp <= 8 {
        println!("Your APS level is: APS 5-8");
    } else if irole == 3 || iexp >=8 && iexp <= 10 {
        println!("Your APS level is: EL1 8-10");
    } else if irole == 4 || iexp >=10 && iexp <= 13 {
        println!("Your APS level is: EL2 10-13");
    } else if irole == 5 || iexp > 13 {
        println!("Your APS level is: SES");
    } else { println!("No APS level available");}
}
fn teacher() {
    let vteacher = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];
    println!("What is your current job description?, Enter:\n0 for Placement\n1 for Classroom Teacher\n2 for Senior Teacher\n3 for Leading Teacher\n4 for Deputy Principal\n5 for Principal");
    let mut irole = String::new();
    io::stdin().read_line(&mut irole).expect("Failed to read input");
    let irole:usize = irole.trim().parse().expect("Invalid input");
    let role = vteacher[irole];
    println!("Your role is {}",role);
    let mut iexp = String::new();
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut iexp).expect("Failed to read input");
    let iexp:u8 = iexp.trim().parse().expect("Invalid input");
    if irole == 0 || iexp >= 1 && iexp <=2 {
        println!("Your APS level is: APS 1-2");
    } else if irole == 1 || iexp >= 3 && iexp <= 5 {
        println!("Your APS level is: APS 3-5");
    } else if irole == 2 || iexp >= 5 && iexp <= 8 {
        println!("Your APS level is: APS 5-8");
    } else if irole == 3 || iexp >=8 && iexp <= 10 {
        println!("Your APS level is: EL1 8-10");
    } else if irole == 4 || iexp >=10 && iexp <= 13 {
        println!("Your APS level is: EL2 10-13");
    } else if irole == 5 || iexp > 13 {
        println!("Your APS level is: SES");
    } else { println!("No APS level available");}
}

fn main() {
    let vsec = vec!["Office Admin","Academic","Lawyer","Teacher"];
    println!("Welcome to the Public Service APS level checker");
    println!("What is your sector?, please enter:\n0 for Office Admin\n1 for Academic\n2 for Laywer\n3 for Teacher");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let indexvsec:usize = input1.trim().parse().expect("Invalid input");
    let sector = vsec[indexvsec] ;
    match sector.trim() {
        "Office Admin" => {
           office_admin()
        }
        "Academic" => {
            academic()
        }
        "Lawyer" => {
            lawyer()
        }
        "Teacher" => {
            teacher()
        }
        _ => {
            println!("Invalid");
        }
    }
   }