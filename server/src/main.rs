use proto::student_service_server::{StudentService, StudentServiceServer};
use proto::{
    CreateStudentRequest, CreateStudentResponse, DeleteStudentRequest, DeleteStudentResponse,
    GetStudentRequest, GetStudentResponse, ListStudentsRequest, ListStudentsResponse, Student,
    UpdateStudentRequest, UpdateStudentResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

type StudentStore = Arc<RwLock<HashMap<String, Student>>>;

#[derive(Debug)]
pub struct StudentServiceImpl {
    store: StudentStore,
}

impl StudentServiceImpl {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Helper method to validate student data
    fn validate_student(&self, student: &Student) -> Result<(), Status> {
        if student.name.trim().is_empty() {
            return Err(Status::invalid_argument("Student name cannot be empty"));
        }
        if student.email.trim().is_empty() {
            return Err(Status::invalid_argument("Student email cannot be empty"));
        }
        if student.age < 0 || student.age > 150 {
            return Err(Status::invalid_argument("Student age must be between 0 and 150"));
        }
        if student.gpa < 0.0 || student.gpa > 4.0 {
            return Err(Status::invalid_argument("Student GPA must be between 0.0 and 4.0"));
        }
        Ok(())
    }
}

#[tonic::async_trait]
impl StudentService for StudentServiceImpl {
    async fn create_student(
        &self,
        request: Request<CreateStudentRequest>,
    ) -> Result<Response<CreateStudentResponse>, Status> {
        let mut student = request.into_inner().student.unwrap_or_default();
        
        // Validate student data
        self.validate_student(&student)?;
        
        // Generate a new ID if not provided
        if student.id.is_empty() {
            student.id = Uuid::new_v4().to_string();
        }

        let mut store = self.store.write().await;
        
        // Check if student already exists
        if store.contains_key(&student.id) {
            return Err(Status::already_exists("Student with this ID already exists"));
        }

        store.insert(student.id.clone(), student.clone());
        
        println!("Created student: {} ({})", student.name, student.id);

        Ok(Response::new(CreateStudentResponse {
            student: Some(student),
        }))
    }

    async fn get_student(
        &self,
        request: Request<GetStudentRequest>,
    ) -> Result<Response<GetStudentResponse>, Status> {
        let student_id = request.into_inner().id;
        
        if student_id.trim().is_empty() {
            return Err(Status::invalid_argument("Student ID cannot be empty"));
        }

        let store = self.store.read().await;
        
        match store.get(&student_id) {
            Some(student) => {
                println!("Retrieved student: {} ({})", student.name, student.id);
                Ok(Response::new(GetStudentResponse {
                    student: Some(student.clone()),
                }))
            }
            None => Err(Status::not_found("Student not found")),
        }
    }

    async fn update_student(
        &self,
        request: Request<UpdateStudentRequest>,
    ) -> Result<Response<UpdateStudentResponse>, Status> {
        let student = request.into_inner().student.unwrap_or_default();
        
        if student.id.trim().is_empty() {
            return Err(Status::invalid_argument("Student ID cannot be empty"));
        }
        
        // Validate student data
        self.validate_student(&student)?;

        let mut store = self.store.write().await;
        
        match store.get_mut(&student.id) {
            Some(existing_student) => {
                *existing_student = student.clone();
                println!("Updated student: {} ({})", student.name, student.id);
                Ok(Response::new(UpdateStudentResponse {
                    student: Some(student),
                }))
            }
            None => Err(Status::not_found("Student not found")),
        }
    }

    async fn delete_student(
        &self,
        request: Request<DeleteStudentRequest>,
    ) -> Result<Response<DeleteStudentResponse>, Status> {
        let student_id = request.into_inner().id;
        
        if student_id.trim().is_empty() {
            return Err(Status::invalid_argument("Student ID cannot be empty"));
        }

        let mut store = self.store.write().await;
        
        match store.remove(&student_id) {
            Some(student) => {
                println!("Deleted student: {} ({})", student.name, student.id);
                Ok(Response::new(DeleteStudentResponse {
                    success: true,
                    message: format!("Student {} deleted successfully", student.name),
                }))
            }
            None => Err(Status::not_found("Student not found")),
        }
    }

    async fn list_students(
        &self,
        request: Request<ListStudentsRequest>,
    ) -> Result<Response<ListStudentsResponse>, Status> {
        let req = request.into_inner();
        let page_size = if req.page_size <= 0 { 10 } else { req.page_size as usize };
        
        let store = self.store.read().await;
        let students: Vec<Student> = store.values().cloned().collect();
        
        // Simple pagination implementation
        let total_count = students.len() as i32;
        let start_index = req.page_token.parse::<usize>().unwrap_or(0);
        let end_index = std::cmp::min(start_index + page_size, students.len());
        
        let page_students = students[start_index..end_index].to_vec();
        let next_page_token = if end_index < students.len() {
            end_index.to_string()
        } else {
            String::new()
        };
        
        println!("Listed {} students (page {}-{})", page_students.len(), start_index, end_index);

        Ok(Response::new(ListStudentsResponse {
            students: page_students,
            next_page_token,
            total_count,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let student_service = StudentServiceImpl::new();

    println!("🎓 Student Management gRPC Server starting on {}", addr);

    Server::builder()
        .add_service(StudentServiceServer::new(student_service))
        .serve(addr)
        .await?;

    Ok(())
}
