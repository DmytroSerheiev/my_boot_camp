mod post {
    // Визначення структури для ідентифікатора поста (ID).
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    // Визначення структури для назви поста.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    // Визначення структури для тіла поста.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}

mod user {
    // Визначення структури для ідентифікатора користувача (ID).
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

// Визначення станів поста: новий, непромодерований, опублікований та видалений.
struct New;
struct Unmoderated;
struct Published;
struct Deleted;

// Визначення структури поста, яка приймає параметр типу S (стан).
#[derive(Clone)]
pub struct Post<S> {
    // Поля поста, такі як ID, ID користувача, назва, тіло та поточний стан.
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: S,
}

impl Post<New> {
    // Функція для створення нового поста.
    pub fn create(id: post::Id, user_id: user::Id, title: post::Title, body: post::Body) -> Self {
        // Повертаємо новий пост зі станом New.
        Self {
            id,
            user_id,
            title,
            body,
            state: New,
        }
    }

    // Функція для публікації поста, яка повертає пост зі станом Unmoderated.
    pub fn publish(self) -> Post<Unmoderated> {
        Post {
            state: Unmoderated,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl Post<Unmoderated> {
    // Функція для схвалення поста, яка повертає пост зі станом Published.
    pub fn approve(self) -> Post<Published> {
        Post {
            state: Published,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }

    // Функція для відхилення поста, яка повертає пост зі станом Deleted.
    pub fn reject(self) -> Post<Deleted> {
        Post {
            state: Deleted,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl Post<Published> {
    // Функція для видалення опублікованого поста, яка повертає пост зі станом Deleted.
    pub fn remove(self) -> Post<Deleted> {
        Post {
            state: Deleted,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_publish_and_remove_post() {
        // Створюємо новий пост.
        let post = Post::create(
            post::Id(1),
            user::Id(1),
            post::Title(String::from("Назва")),
            post::Body(String::from("Тіло")),
        );

        // Публікуємо пост, отримуємо непромодерований пост.
        let unmoderated_post = post.publish();
        // Схвалюємо пост, отримуємо опублікований пост.
        let published_post = unmoderated_post.approve();
        // Видаляємо опублікований пост, отримуємо пост зі станом Deleted.
        let removed_post = published_post.remove();
    }

    #[test]
    fn should_reject_post_on_moderation() {
        // Створюємо новий пост.
        let post = Post::create(
            post::Id(1),
            user::Id(1),
            post::Title(String::from("Назва")),
            post::Body(String::from("Тіло")),
        );

        // Публікуємо пост, отримуємо непромодерований пост.
        let unmoderated_post = post.publish();
        // Відхиляємо пост на модерації, отримуємо пост зі станом Deleted.
        let rejected_post = unmoderated_post.reject();
    }
}
