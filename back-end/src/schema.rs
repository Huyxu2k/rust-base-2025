// @generated automatically by Diesel CLI.

diesel::table! {
    _access_tokens (ID) {
        ID -> Integer,
        UserID -> Integer,
        AccessToken -> Text,
        Expiry -> Datetime,
        #[max_length = 255]
        IPAddress -> Nullable<Varchar>,
        UserAgent -> Nullable<Text>,
        CreatedAt -> Nullable<Datetime>,
        Revoked -> Nullable<Bool>,
    }
}

diesel::table! {
    _client (ID) {
        ID -> Integer,
        #[max_length = 255]
        Name -> Varchar,
        #[max_length = 255]
        ClientID -> Varchar,
        #[max_length = 255]
        ClientSecret -> Varchar,
        #[max_length = 255]
        RedirectUri -> Nullable<Varchar>,
        IsConfidential -> Nullable<Bool>,
        Enabled -> Nullable<Bool>,
    }
}

diesel::table! {
    _employees (ID) {
        ID -> Integer,
        #[max_length = 50]
        FirstName -> Varchar,
        #[max_length = 50]
        MiddleName -> Nullable<Varchar>,
        #[max_length = 50]
        LastName -> Varchar,
        Birthday -> Nullable<Date>,
        #[max_length = 50]
        IdentificationNumber -> Varchar,
        #[max_length = 50]
        PhoneNumber -> Nullable<Varchar>,
        CreatedBy -> Integer,
        UpdatedBy -> Integer,
        CreatedAt -> Nullable<Datetime>,
        UpdatedAt -> Nullable<Datetime>,
    }
}

diesel::table! {
    _group_roles (ID) {
        ID -> Integer,
        GroupID -> Integer,
        RoleID -> Integer,
    }
}

diesel::table! {
    _groups (ID) {
        ID -> Integer,
        #[max_length = 255]
        GroupName -> Varchar,
        Description -> Nullable<Text>,
        CreatedBy -> Integer,
        UpdatedBy -> Integer,
        CreatedAt -> Nullable<Datetime>,
        UpdatedAt -> Nullable<Datetime>,
    }
}

diesel::table! {
    _http_methods (MethodID) {
        MethodID -> Integer,
        #[max_length = 10]
        MethodName -> Varchar,
    }
}

diesel::table! {
    _permissions (ID) {
        ID -> Integer,
        #[max_length = 255]
        APIEndpoint -> Varchar,
        Description -> Nullable<Text>,
    }
}

diesel::table! {
    _refresh_tokens (ID) {
        ID -> Integer,
        UserID -> Integer,
        #[max_length = 255]
        UUID -> Varchar,
        RefreshToken -> Text,
        Expiry -> Datetime,
        #[max_length = 255]
        IPAddress -> Nullable<Varchar>,
        UserAgent -> Nullable<Text>,
        CreatedAt -> Nullable<Datetime>,
        Revoked -> Nullable<Bool>,
    }
}

diesel::table! {
    _role_permissions (ID) {
        ID -> Integer,
        RoleID -> Integer,
        PermissionID -> Integer,
    }
}

diesel::table! {
    _roles (ID) {
        ID -> Integer,
        #[max_length = 50]
        RoleName -> Varchar,
        Description -> Nullable<Text>,
        CreatedBy -> Integer,
        UpdatedBy -> Integer,
        CreatedAt -> Nullable<Datetime>,
        UpdatedAt -> Nullable<Datetime>,
    }
}

diesel::table! {
    _user_groups (ID) {
        ID -> Integer,
        UserID -> Integer,
        GroupID -> Integer,
    }
}

diesel::table! {
    _user_permissions (ID) {
        ID -> Integer,
        UserID -> Integer,
        PermissionID -> Integer,
    }
}

diesel::table! {
    _user_roles (ID) {
        ID -> Integer,
        UserID -> Integer,
        RoleID -> Integer,
    }
}

diesel::table! {
    _users (ID) {
        ID -> Integer,
        EmployeeId -> Nullable<Integer>,
        #[max_length = 50]
        Username -> Varchar,
        #[max_length = 255]
        PasswordHash -> Varchar,
        #[max_length = 255]
        Email -> Varchar,
        EmailVerified -> Nullable<Bool>,
        IsActive -> Nullable<Bool>,
        CreatedBy -> Integer,
        UpdatedBy -> Integer,
        CreatedAt -> Nullable<Datetime>,
        UpdatedAt -> Nullable<Datetime>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _access_tokens,
    _client,
    _employees,
    _group_roles,
    _groups,
    _http_methods,
    _permissions,
    _refresh_tokens,
    _role_permissions,
    _roles,
    _user_groups,
    _user_permissions,
    _user_roles,
    _users,
);
