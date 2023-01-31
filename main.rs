use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    /*Part 1*/

    #[derive(Debug, PartialEq)]
    //struct to contain a students name and a vector of all their grades
    pub struct StudentGrades {
        name: String,
        grades: Vec<i64>,
    }

    impl StudentGrades {
        //allows for initialization of new student through a function
        fn create_student(create_name: String, create_grades: Vec<i64>) -> StudentGrades {
            StudentGrades {name: create_name, grades: create_grades,}
        }
        //iterate through vector of students grades, adds them, then divide by the size to return the average of the student grades
        fn find_average(&self) -> i64 {
            let sum: i64 = self.grades.iter().sum();
            let average = sum / self.grades.len() as i64;
            return average;
        }
        //use previous function to get the average, then return a letter grade based on the average
        fn find_letter_grade(&self) -> char {
            let average = self.find_average();
            if average >= 90 {
                return 'A';
            } else if average < 90 && average >= 80 {
                return 'B';
            } else if average < 80 && average >= 70 {
                return 'C';
            } else if average < 70 && average >= 60 {
                return 'D';
            } else {
                return 'F';
            }
        }
    }
    
    //unit tests for part 1, setting 3 students grades and testing the function returns for their averages and letter grades, program will error if incorrect
    let student1 = StudentGrades::create_student("Brett".to_string(), vec![90, 95, 100]);
    assert_eq!(95, student1.find_average());
    assert_eq!('A', student1.find_letter_grade());

    let student2 = StudentGrades::create_student("Brett".to_string(), vec![50, 70, 80, 100]);
    assert_eq!(75, student2.find_average());
    assert_eq!('C', student2.find_letter_grade());

    let student3 = StudentGrades::create_student("Brett".to_string(), vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    assert_eq!(55, student3.find_average());
    assert_eq!('F', student3.find_letter_grade());
    

    /*Part 2*/

    #[derive(Debug, PartialEq)]
    //struct to hold a vector of students and their names and grades
    struct CourseGrades {
        students: Vec<StudentGrades>,
    }

    impl CourseGrades {
        //read from a file path (test.txt) and store the name and vector of grades in StudentGrade objects
        fn from_file(file_path: String) -> CourseGrades {
            //file reading to get line by line
            let file = File::open(file_path).expect("Cannot Open File");
            let reader = BufReader::new(file);
            let mut new_student_vec: Vec<StudentGrades> = Vec::new();

            //loop through line by line, because it is assumed the file will have the format of name, then grades, with each student separated by line
            for (_index, line) in reader.lines().enumerate() {
                let mut i = 1;  //tracks first word of line to know that it should be saved as a string because it is a name
                let mut name_vec = " ".to_string();
                let mut grades_vec = Vec::<i64>::new();
                let mut line = line.unwrap();
                line = line.replace(",", " ");      //remove all commas so tokens can be scanned by whitespace

                for token in line.split_whitespace() {
                    if i == 1 { // add name to string
                        name_vec = token.to_string();
                        i = i + 1;
                    } else { //adds all grades to vector
                        grades_vec.push(token.parse::<i64>().unwrap());
                    }
                }
                new_student_vec.push(StudentGrades::create_student(name_vec, grades_vec));  //push the created student to the vector of students held in CourseGrades
            }
            CourseGrades {students: new_student_vec,}   //return the struct of all student grades
        }

        //retrive student based on name
        fn get_student(&self, student_name : String) -> Option<&StudentGrades> {
            //loop through every student
            for current_student in &self.students{
                if student_name == current_student.name{
                    return Some(current_student);   //return the student who has a matching name
                }
            }
            return None;        //return none if the name does not match
        }
        //get the average of a certain test score for all the students
        fn get_average(&self, n : usize)-> Option<i64> {
            if n > self.students.len(){
                return None;    //if searching for the nth test grade, but there is not that many grades, return none as the average does not exist
            }
            //iterate and add every nth grade from the students 
            let mut sum:i64 = 0;
            for current_student in &self.students{
                sum = sum + current_student.grades[n - 1];  // -1 for accessing properly
            }
            let average = sum / self.students.len() as i64;
            return Some(average);   //return the average
        }
    }

    //unit tests for part 2, checks if return from function are expected for the file, error if false
    let all_students = CourseGrades::from_file("test.txt".to_string());     //read from file and store contents into struct
    assert_eq!(all_students.get_student("jim".to_string()).unwrap(), &StudentGrades { name: "jim".to_string(), grades: vec![78, 89, 95] });
    assert_eq!(all_students.get_student("not_a_name".to_string()), None);

    assert_eq!(all_students.get_average(2).unwrap(), 66);
    assert_eq!(all_students.get_average(7), None);
    

    /*Part 3 */

    use serde::Deserialize;
    use std::fs;

    #[derive(Deserialize, Debug, PartialEq)]
    #[allow(non_snake_case)]
    //variable names must match file headers, so snake case is not an option unless file headers are changed
    struct CourseStruct {
        Course: String,
        Title: String,
        Instructor: String,
        StartTime: String,
        EndTime: String,
        Days: String,
    }

    #[derive(Debug, PartialEq)]
    //vector of every course in file
    struct CourseSchedule {
        courses: Vec<CourseStruct>,
    }

    impl CourseSchedule {
        //reads through search.csv to populate struct CourseSchedule with a vector of classes in file, returns all the classes in vector in a struct
        fn from_csv_file() -> CourseSchedule {
            let file_path = "search.csv";
            let csv = fs::read_to_string(file_path).expect("Failed to read file");

            let mut reader = csv::Reader::from_reader(csv.as_bytes());
            let mut courses_vec: Vec<CourseStruct> = Vec::new();

            for course_struct in reader.deserialize() {
                let course_struct: CourseStruct = course_struct.expect("error looping");
                courses_vec.push(CourseStruct {
                    Course: course_struct.Course,
                    Title: course_struct.Title,
                    Instructor: course_struct.Instructor,
                    StartTime: course_struct.StartTime,
                    EndTime: course_struct.EndTime,
                    Days: course_struct.Days,
                });
            }
            return CourseSchedule {courses: courses_vec,};
        }
        //finds courses a selected times
        fn courses_at(full_schedule: CourseSchedule, time: &str, days: &str) -> Option<Vec<CourseStruct>> {
            let mut found_courses:Vec<CourseStruct> = Vec::new();
            //loop through every class and see if start time and day match the search
            for current_course in full_schedule.courses{
                if current_course.StartTime == time.to_string() && current_course.Days == days.to_string(){
                    found_courses.push(current_course);
                }
            }
            //at least one course was found to be returned
            if found_courses.len() > 0{
                return Some(found_courses);
            }
            //no class at searched time/day
            return None;
        }
    }
    
    //unit tests for part 3, returns classes at start time listed, error if false
    let file_contents1 = CourseSchedule::from_csv_file();    //fill in struct with file contents to be used in tests
    let file_contents2 = CourseSchedule::from_csv_file();    //fill in struct with file contents to be used in tests

    //find classes at specified times, some have duplicates in file
    let search1 = CourseSchedule::courses_at(file_contents1, "1100", "TTH").unwrap(); 
    assert_eq!(search1[0], CourseStruct { Course: "100".to_string(), Title: "COMPTR SCIENCE PRINCIPLES".to_string(), Instructor: "P. KRAFT".to_string(), StartTime: "1100".to_string(), EndTime: "1215".to_string(), Days: "TTH".to_string() });
    assert_eq!(search1[1], CourseStruct { Course: "549".to_string(), Title: "MACHINE LEARNING".to_string(), Instructor: "Y. XU".to_string(), StartTime: "1100".to_string(), EndTime: "1215".to_string(), Days: "TTH".to_string()});
    assert_eq!(search1[2], CourseStruct { Course: "549".to_string(), Title: "MACHINE LEARNING".to_string(), Instructor: "Y. XU".to_string(), StartTime: "1100".to_string(), EndTime: "1215".to_string(), Days: "TTH".to_string()});

    let search2 = CourseSchedule::courses_at(file_contents2, "1230", "TTH").unwrap();
    assert_eq!(search2[0], CourseStruct { Course: "480".to_string(), Title: "OPERATING SYSTEMS".to_string(), Instructor: "J. CARROLL".to_string(), StartTime: "1230".to_string(), EndTime: "1345".to_string(), Days: "TTH".to_string() });
    assert_eq!(search2[1], CourseStruct { Course: "480".to_string(), Title: "OPERATING SYSTEMS".to_string(), Instructor: "J. CARROLL".to_string(), StartTime: "1230".to_string(), EndTime: "1345".to_string(), Days: "TTH".to_string() });

}

