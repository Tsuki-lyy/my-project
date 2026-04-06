# OOA-OOP实践过程报告

## 第4周实验：UML建模与OOA-OOP实践

### 1. 用例图

```mermaid
useCaseDiagram
    title 高校图书借阅系统用例图
    
    actor Student as 学生
    actor Teacher as 教师
    actor Librarian as 图书管理员
    
    usecase Login as 登录
    usecase QueryBook as 查询图书
    usecase BorrowBook as 借书
    usecase ReturnBook as 还书
    usecase ManageBook as 管理图书
    usecase ReserveBook as 预约图书
    usecase ViewHistory as 查看借阅历史
    usecase CalculateFine as 计算罚款
    usecase ViewAllRecords as 查看所有借阅记录
    
    Student -- Login
    Student -- QueryBook
    Student -- BorrowBook
    Student -- ReturnBook
    Student -- ReserveBook
    Student -- ViewHistory
    
    Teacher -- Login
    Teacher -- QueryBook
    Teacher -- BorrowBook
    Teacher -- ReturnBook
    Teacher -- ReserveBook
    Teacher -- ViewHistory
    
    Librarian -- Login
    Librarian -- ManageBook
    Librarian -- ViewAllRecords
    
    BorrowBook --|> QueryBook : <<include>>
    ReturnBook --|> QueryBook : <<include>>
    ReturnBook --|> CalculateFine : <<include>>
    ReserveBook --|> BorrowBook : <<extend>>
```

### 2. 借书活动图

```mermaid
flowchart TD
    subgraph 借书流程活动图
        A[开始] --> B["用户登录"]
        B --> C["查询图书"]
        C --> D["选择图书"]
        D --> E["检查库存"]
        E --> F{"用户是否有效？"}
        
        F -->|是| G{"是否已达借阅上限？"}
        F -->|否| H["返回错误：用户无效"]
        H --> Z[结束]
        
        G -->|否| I{"图书是否可借？"}
        G -->|是| J["返回错误：已达借阅上限"]
        J --> Z
        
        I -->|是| K["执行借阅"]
        I -->|否| L["返回错误：图书不可借"]
        L --> Z
        
        K --> M["记录借阅"]
        M --> N["返回结果"]
        N --> Z
    end
```

### 3. 还书活动图

```mermaid
flowchart TD
    subgraph 还书流程活动图
        A[开始] --> B["提交还书请求"]
        B --> C["验证借阅记录"]
        C --> D{"是否超期？"}
        
        D -->|是| E["计算罚款\n(每天0.1元，最高不超过图书原价)"]
        D -->|否| F["跳过罚款计算"]
        
        E --> G["生成罚单"]
        G --> H["更新图书状态"]
        
        F --> H
        
        H --> I["记录还书"]
        I --> J["通知用户"]
        J --> Z[结束]
    end
```

### 4. 借书顺序图

```mermaid
sequenceDiagram
    participant User as 用户
    participant BC as BorrowController
    participant US as UserService
    participant BR as BorrowRepository
    participant BS as BookService
    participant BCR as BookCopyRepository
    
    User->>BC: requestBorrow(userId, copyId)
    
    BC->>US: getUserById(userId)
    US->>UserRepository: findById(userId)
    alt 用户无效
        UserRepository-->>US: return None
        US-->>BC: return None
        BC-->>User: return 错误：用户无效
    else 用户有效
        UserRepository-->>US: return User
        US-->>BC: return User
        
        BC->>US: checkBorrowLimit(userId)
        US->>BR: countByUserId(userId)
        BR-->>US: return count
        
        alt 已达借阅上限
            US-->>BC: return false
            BC-->>User: return 错误：已达借阅上限
        else 未达借阅上限
            US-->>BC: return true
            
            BC->>BS: getBookCopyById(copyId)
            BS->>BCR: findById(copyId)
            
            alt 图书不可借
                BCR-->>BS: return None
                BS-->>BC: return None
                BC-->>User: return 错误：图书不可借
            else 图书可借
                BCR-->>BS: return BookCopy
                BS-->>BC: return BookCopy
                
                BC->>BorrowService: confirmBorrow(userId, copyId)
                BorrowService->>BR: save(borrowRecord)
                BR-->>BorrowService: return record
                BorrowService-->>BC: return success
                BC-->>User: return 成功
            end
        end
    end
```

### 5. 还书顺序图

```mermaid
sequenceDiagram
    participant User as 用户
    participant RC as ReturnController
    participant UR as UserRepository
    participant BR as BorrowRepository
    participant FS as FineService
    participant BS as BookService
    
    User->>RC: requestReturn(userId, copyId)
    
    RC->>UR: getUserById(userId)
    alt 用户不存在
        UR-->>RC: return None
        RC-->>User: return 错误：用户不存在
    else 用户存在
        UR-->>RC: return User
        
        RC->>BR: getBorrowRecord(copyId)
        alt 借阅记录不存在
            BR-->>RC: return None
            RC-->>User: return 错误：借阅记录不存在
        else 借阅记录存在
            BR-->>RC: return BorrowRecord
            
            RC->>FS: calculateFine(recordId)
            FS->>BR: getBorrowDate()
            BR-->>FS: return borrowDate
            
            alt 超期
                FS-->>RC: return fineAmount(>0)
            else 不超期
                FS-->>RC: return fineAmount(0)
            end
            
            RC->>BS: updateStatus(copyId, Available)
            RC->>BR: createReturnRecord()
            BR-->>RC: return record
            RC-->>User: return result
        end
    end
```

### 6. 类图

```mermaid
classDiagram
    direction LR
    
    class User {
        <<abstract>>
        +id: String
        +name: String
        +userType: String
        +maxBorrowLimit: u32
        +status: String
        +canBorrow(): bool
    }
    
    class Student {
        +studentId: String
        +major: String
    }
    
    class Teacher {
        +teacherId: String
        +department: String
    }
    
    class Book {
        +isbn: String
        +title: String
        +author: String
        +publisher: String
    }
    
    class BookCopy {
        +copyId: String
        +bookId: String
        +status: String
        +location: String
        +isAvailable(): bool
    }
    
    class BorrowRecord {
        +recordId: String
        +userId: String
        +copyId: String
        +borrowDate: NaiveDate
        +dueDate: NaiveDate
        +returnDate: Option<NaiveDate>
        +status: String
        +isOverdue(): bool
    }
    
    class Reservation {
        +reservationId: String
        +userId: String
        +copyId: String
        +reservationDate: NaiveDate
        +status: String
    }
    
    class BorrowService {
        +requestBorrow(userId: String, copyId: String): Result<String, String>
    }
    
    class BookService {
        +getBookCopy(copyId: String): Option<BookCopy>
        +updateStatus(copyId: String, status: String): Result<(), String>
    }
    
    class UserService {
        +getUser(userId: String): Option<User>
        +checkBorrowLimit(userId: String): bool
    }
    
    class FineService {
        +calculateFine(recordId: String): f64
        +isOverdue(borrowDate: NaiveDate, returnDate: NaiveDate): bool
    }
    
    class BorrowController {
        +requestBorrow(userId: String, copyId: String): Result<String, String>
    }
    
    class ReturnController {
        +requestReturn(userId: String, copyId: String): Result<String, String>
    }
    
    class UserRepository {
        <<interface>>
        +findById(userId: String): Option<User>
        +countBorrow(userId: String): usize
    }
    
    class BookCopyRepository {
        <<interface>>
        +findById(copyId: String): Option<BookCopy>
        +updateStatus(copyId: String, status: String): Result<(), String>
    }
    
    class BorrowRecordRepository {
        <<interface>>
        +save(record: BorrowRecord): Result<String, String>
        +findByCopyId(copyId: String): Option<BorrowRecord>
        +createReturnRecord(record: BorrowRecord): Result<BorrowRecord, String>
    }
    
    User <|-- Student
    User <|-- Teacher
    
    User "1" -- "*" BorrowRecord : has
    Book "1" -- "*" BookCopy : has
    BookCopy "1" -- "*" BorrowRecord : has
    User "1" -- "*" Reservation : has
    BookCopy "1" -- "*" Reservation : has
    
    BorrowController --> BorrowService
    ReturnController --> FineService
    ReturnController --> BookService
    ReturnController --> BorrowRecordRepository
    
    BorrowService --> UserService
    BorrowService --> BookService
    BorrowService --> BorrowRecordRepository
    UserService --> UserRepository
    BookService --> BookCopyRepository
    FineService --> BorrowRecordRepository
    
    BorrowService ..> UserRepository : uses
    BorrowService ..> BookCopyRepository : uses
    BorrowService ..> BorrowRecordRepository : uses
    FineService ..> BorrowRecordRepository : uses
```

### 7. 状态图

#### BookCopy状态图

```mermaid
stateDiagram-v2
    [*] --> Available
    
    state Available {
        [*] --> Available
    }
    
    state Borrowed {
        [*] --> Borrowed
    }
    
    state Reserved {
        [*] --> Reserved
    }
    
    state Lost {
        [*] --> Lost
    }
    
    state Discarded {
        [*] --> Discarded
    }
    
    Available --> Borrowed : borrow
    Available --> Reserved : reserve
    Reserved --> Available : cancelReserve
    Reserved --> Borrowed : borrow
    Borrowed --> Available : return
    Borrowed --> Lost : reportLost
    Lost --> Available : found
    
    Available --> Discarded : discard
    Borrowed --> Discarded : discard
    Reserved --> Discarded : discard
    Lost --> Discarded : discard
    Discarded --> [*]
```

#### BorrowRecord状态图

```mermaid
stateDiagram-v2
    [*] --> Borrowing : borrow
    
    state Borrowing {
        [*] --> Borrowing
    }
    
    state Returned {
        [*] --> Returned
    }
    
    state Overdue {
        [*] --> Overdue
    }
    
    Borrowing --> Returned : return
    Borrowing --> Overdue : overdue
    
    Returned --> [*]
    Overdue --> [*]
```

### 8. 设计决策说明

#### 8.1 架构设计

采用三层架构（Controller-Service-Repository）：

- **Controller层**：处理用户请求，调用Service层，返回响应
- **Service层**：实现业务逻辑，调用Repository层
- **Repository层**：负责数据存储和访问

#### 8.2 设计模式应用

1. **依赖倒置原则**：Service层依赖Repository接口，而不是具体实现
2. **单一职责原则**：每个类只负责一个功能领域
3. **开放封闭原则**：通过接口扩展，而不是修改现有代码

#### 8.3 实体关系设计

- **User与Student/Teacher**：泛化关系，Student和Teacher继承User
- **Book与BookCopy**：1对多关系，一本书可以有多个副本
- **User与BorrowRecord**：1对多关系，一个用户可以有多个借阅记录
- **BookCopy与BorrowRecord**：1对多关系，一个副本可以有多个借阅记录

#### 8.4 状态管理

- **BookCopy**：使用状态机管理图书副本的生命周期（Available→Borrowed→Available等）
- **BorrowRecord**：使用状态机管理借阅记录的状态（Borrowing→Returned/Overdue）

#### 8.5 业务规则

- 学生借阅限制：5本
- 教师借阅限制：10本
- 逾期罚款：每天0.1元，最高不超过图书原价
- 预约优先：预约的图书优先借给预约用户

#### 8.6 错误处理

在顺序图中体现了完整的错误处理流程：
- 用户不存在时返回错误
- 借阅记录不存在时返回错误
- 已达借阅上限时返回错误
- 图书不可借时返回错误

#### 8.7 扩展性考虑

- 通过接口设计支持不同的数据存储实现
- 通过状态机模式支持未来可能的新状态和转换
- 通过Service层的抽象支持业务规则的扩展
