

// mod check {
//     schema!{
//         #[use id, times]
//         User {
//             name: TEXT,
//             password: VARCHAR(20),
//         },
//         #[use times, id]
//         Task {
//             title: VARCHAR(20) where NOT_NULL + DEFAULT("No title"),
//             description: TEXT where NOT_NULL + DEFAULT("") + CHECK(id > 0),
//         } where UNIQUE(title, password) + PRIMARY_KEY(title, description)
//     }
// }
