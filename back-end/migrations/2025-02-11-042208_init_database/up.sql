CREATE TABLE IF NOT EXISTS _Employees (
  ID INT PRIMARY KEY AUTO_INCREMENT,   
  FirstName NVARCHAR(50) NOT NULL,
  MiddleName NVARCHAR(50),
  LastName NVARCHAR(50) NOT NULL,
  Birthday DATE,
  IdentificationNumber VARCHAR(50) NOT NULL UNIQUE,
  PhoneNumber VARCHAR(50),
  CreatedBy INT NOT NULL,
  UpdatedBy INT NOT NULL,
  CreatedAt DATETIME DEFAULT CURRENT_TIMESTAMP,
  UpdatedAt DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS _Users (
    ID INT PRIMARY KEY AUTO_INCREMENT,     
    EmployeeId INT,
    Username VARCHAR(50) NOT NULL UNIQUE,        
    PasswordHash VARCHAR(255) NOT NULL,          
    Email VARCHAR(255) NOT NULL UNIQUE,
    EmailVerified BOOLEAN DEFAULT FALSE,
    IsActive BOOLEAN DEFAULT TRUE, 
    CreatedBy INT NOT NULL,
    UpdatedBy INT NOT NULL,
    CreatedAt DATETIME DEFAULT CURRENT_TIMESTAMP,
    UpdatedAt DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS _User_Roles(
   ID INT PRIMARY KEY AUTO_INCREMENT,
   UserID INT NOT NULL,
   RoleID INT NOT NULL,
   UNIQUE(UserID,RoleID)
);

CREATE TABLE IF NOT EXISTS _User_Groups (
    ID INT PRIMARY KEY AUTO_INCREMENT,
    UserID INT NOT NULL,
    GroupID INT NOT NULL
);

CREATE TABLE IF NOT EXISTS _Groups (
    ID INT PRIMARY KEY AUTO_INCREMENT,        
    GroupName VARCHAR(255) NOT NULL UNIQUE,                                  
    Description TEXT,                              
    CreatedBy INT NOT NULL,
    UpdatedBy INT NOT NULL,
    CreatedAt DATETIME DEFAULT CURRENT_TIMESTAMP,
    UpdatedAt DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS _Group_Roles (
    ID INT PRIMARY KEY AUTO_INCREMENT,
    GroupID INT NOT NULL,                       
    RoleID INT NOT NULL                    
);

CREATE TABLE IF NOT EXISTS _Roles (
    ID INT PRIMARY KEY AUTO_INCREMENT,        
    RoleName VARCHAR(50) NOT NULL UNIQUE,          
    Description TEXT,
    CreatedBy INT NOT NULL,
    UpdatedBy INT NOT NULL,
    CreatedAt DATETIME DEFAULT CURRENT_TIMESTAMP,
    UpdatedAt DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS _Role_Permissions (
	ID INT PRIMARY KEY AUTO_INCREMENT,
    RoleID INT NOT NULL,
    PermissionID INT NOT NULL
);
CREATE TABLE IF NOT EXISTS _User_Permissions (
    ID INT PRIMARY KEY AUTO_INCREMENT,
    UserID INT NOT NULL,                           
    PermissionID INT NOT NULL
);

CREATE TABLE IF NOT EXISTS _Permissions (
    ID INT PRIMARY KEY AUTO_INCREMENT,
    APIEndpoint VARCHAR(255) NOT NULL,
    Description TEXT
);

CREATE TABLE IF NOT EXISTS _Http_Methods (
    MethodID INT PRIMARY KEY AUTO_INCREMENT,
    MethodName VARCHAR(10) NOT NULL UNIQUE         
);

CREATE TABLE IF NOT EXISTS _Access_Tokens (
    ID INT PRIMARY KEY AUTO_INCREMENT,        
    UserID INT NOT NULL,                           
    AccessToken TEXT NOT NULL,                     
    Expiry DATETIME NOT NULL,                     
    IPAddress VARCHAR(255) DEFAULT NULL,           
    UserAgent TEXT DEFAULT NULL,                   
    CreatedAt DATETIME DEFAULT CURRENT_TIMESTAMP ,
    Revoked BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS _Refresh_Tokens (
    ID INT PRIMARY KEY AUTO_INCREMENT,         
    UserID INT NOT NULL,                            
    RefreshToken TEXT NOT NULL,                     
    Expiry DATETIME NOT NULL,                      
    IPAddress VARCHAR(255) DEFAULT NULL,            
    UserAgent TEXT DEFAULT NULL,                    
    CreatedAt DATETIME DEFAULT CURRENT_TIMESTAMP,
    Revoked BOOLEAN DEFAULT FALSE                 
);

CREATE TABLE IF NOT EXISTS _Client(
    ID INT PRIMARY KEY AUTO_INCREMENT,
    Name VARCHAR(255) NOT NULL,
    ClientID VARCHAR(255) NOT NULL,
    ClientSecret VARCHAR(255) NOT NULL,
    RedirectUri VARCHAR(255) DEFAULT NULL,
    IsConfidential BOOLEAN DEFAULT FALSE,
    Enabled BOOLEAN DEFAULT TRUE
);