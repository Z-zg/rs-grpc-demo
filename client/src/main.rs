use proto::student_service_client::StudentServiceClient;
use proto::{
    CreateStudentRequest, DeleteStudentRequest, GetStudentRequest, ListStudentsRequest, Student,
    UpdateStudentRequest,
};
use tonic::transport::Channel;

type StudentClient = StudentServiceClient<Channel>;

async fn create_sample_students(client: &mut StudentClient) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("\n🎓 Creating sample students...");
    
    let students = vec![
        Student {
            id: String::new(), // Will be auto-generated
            name: "Alice Johnson".to_string(),
            email: "alice.johnson@university.edu".to_string(),
            age: 20,
            major: "Computer Science".to_string(),
            gpa: 3.8,
        },
        Student {
            id: String::new(),
            name: "Bob Smith".to_string(),
            email: "bob.smith@university.edu".to_string(),
            age: 22,
            major: "Mathematics".to_string(),
            gpa: 3.6,
        },
        Student {
            id: String::new(),
            name: "Carol Davis".to_string(),
            email: "carol.davis@university.edu".to_string(),
            age: 19,
            major: "Physics".to_string(),
            gpa: 3.9,
        },
    ];

    let mut created_ids = Vec::new();

    for student in students {
        let request = tonic::Request::new(CreateStudentRequest {
            student: Some(student.clone()),
        });

        match client.create_student(request).await {
            Ok(response) => {
                let created_student = response.into_inner().student.unwrap();
                println!("✅ Created: {} (ID: {})", created_student.name, created_student.id);
                created_ids.push(created_student.id);
            }
            Err(e) => {
                println!("❌ Failed to create student {}: {}", student.name, e);
            }
        }
    }

    Ok(created_ids)
}

async fn demonstrate_get_student(client: &mut StudentClient, student_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Getting student by ID: {}", student_id);
    
    let request = tonic::Request::new(GetStudentRequest {
        id: student_id.to_string(),
    });

    match client.get_student(request).await {
        Ok(response) => {
            let student = response.into_inner().student.unwrap();
            println!("✅ Found student:");
            println!("   Name: {}", student.name);
            println!("   Email: {}", student.email);
            println!("   Age: {}", student.age);
            println!("   Major: {}", student.major);
            println!("   GPA: {:.2}", student.gpa);
        }
        Err(e) => {
            println!("❌ Failed to get student: {}", e);
        }
    }

    Ok(())
}

async fn demonstrate_update_student(client: &mut StudentClient, student_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 Updating student: {}", student_id);
    
    // First get the current student
    let get_request = tonic::Request::new(GetStudentRequest {
        id: student_id.to_string(),
    });

    let current_student = match client.get_student(get_request).await {
        Ok(response) => response.into_inner().student.unwrap(),
        Err(e) => {
            println!("❌ Failed to get student for update: {}", e);
            return Ok(());
        }
    };

    // Update the student's GPA and major
    let updated_student = Student {
        id: current_student.id,
        name: current_student.name,
        email: current_student.email,
        age: current_student.age,
        major: "Computer Engineering".to_string(), // Changed major
        gpa: 3.95, // Improved GPA
    };

    let update_request = tonic::Request::new(UpdateStudentRequest {
        student: Some(updated_student.clone()),
    });

    match client.update_student(update_request).await {
        Ok(response) => {
            let student = response.into_inner().student.unwrap();
            println!("✅ Updated student:");
            println!("   Name: {}", student.name);
            println!("   Major: {} (changed)", student.major);
            println!("   GPA: {:.2} (improved)", student.gpa);
        }
        Err(e) => {
            println!("❌ Failed to update student: {}", e);
        }
    }

    Ok(())
}

async fn demonstrate_list_students(client: &mut StudentClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Listing all students...");
    
    let request = tonic::Request::new(ListStudentsRequest {
        page_size: 10,
        page_token: String::new(),
    });

    match client.list_students(request).await {
        Ok(response) => {
            let response = response.into_inner();
            println!("✅ Found {} students (total: {}):", response.students.len(), response.total_count);
            
            for (i, student) in response.students.iter().enumerate() {
                println!("   {}. {} - {} (GPA: {:.2})", 
                    i + 1, student.name, student.major, student.gpa);
            }
            
            if !response.next_page_token.is_empty() {
                println!("   (More students available - next page token: {})", response.next_page_token);
            }
        }
        Err(e) => {
            println!("❌ Failed to list students: {}", e);
        }
    }

    Ok(())
}

async fn demonstrate_delete_student(client: &mut StudentClient, student_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🗑️  Deleting student: {}", student_id);
    
    let request = tonic::Request::new(DeleteStudentRequest {
        id: student_id.to_string(),
    });

    match client.delete_student(request).await {
        Ok(response) => {
            let response = response.into_inner();
            if response.success {
                println!("✅ {}", response.message);
            } else {
                println!("❌ Failed to delete student: {}", response.message);
            }
        }
        Err(e) => {
            println!("❌ Failed to delete student: {}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Student Management gRPC Client Demo");
    
    // Connect to the server
    let mut client = StudentServiceClient::connect("http://[::1]:50051").await?;
    println!("✅ Connected to gRPC server");

    // Demonstrate all CRUD operations
    
    // 1. Create students
    let student_ids = create_sample_students(&mut client).await?;
    
    if student_ids.is_empty() {
        println!("❌ No students were created successfully");
        return Ok(());
    }

    // 2. List all students
    demonstrate_list_students(&mut client).await?;

    // 3. Get a specific student
    demonstrate_get_student(&mut client, &student_ids[0]).await?;

    // 4. Update a student
    if student_ids.len() > 1 {
        demonstrate_update_student(&mut client, &student_ids[1]).await?;
    }

    // 5. List students again to see the update
    demonstrate_list_students(&mut client).await?;

    // 6. Delete a student
    if student_ids.len() > 2 {
        demonstrate_delete_student(&mut client, &student_ids[2]).await?;
    }

    // 7. Final list to see the deletion
    demonstrate_list_students(&mut client).await?;

    println!("\n🎉 Demo completed successfully!");
    
    Ok(())
}
