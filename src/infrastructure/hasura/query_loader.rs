use include_dir::Dir;
use std::io::{self, Error, ErrorKind};

pub struct GraphQLDLoader<'a> {
    dir: &'a Dir<'a>,
}

impl<'a> GraphQLDLoader<'a> {
    /// Создаёт загрузчик GraphQL-запросов из включённой директории
    ///
    pub fn new(dir: &'a Dir<'a>) -> Self {
        Self { dir }
    }

    /// Читает содержимое `.graphql` файла из включённой директории
    pub fn read_query(&self, filename: &str) -> io::Result<String> {
        // Проверка расширения
        if !filename.ends_with(".graphql") {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Filename must end with .graphql",
            ));
        }

        // Ищем файл по имени
        match self.dir.get_file(filename) {
            Some(file) => {
                let content = file
                    .contents_utf8()
                    .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid UTF-8 in file"))?;
                Ok(content.to_string())
            }
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("File '{}' not found", filename),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::{include_dir, Dir};

    // Подключаем тестовую директорию (у вас должна быть папка `tests/graphql` с .graphql-файлами)
    static TEST_GQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/tests/graphql");

    #[test]
    fn test_read_existing_query() {
        let loader = GraphQLDLoader::new(&TEST_GQL_DIR);
        let result = loader.read_query("test_query.graphql");

        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content.contains("query")); // или конкретный текст
    }

    #[test]
    fn test_read_nonexistent_file() {
        let loader = GraphQLDLoader::new(&TEST_GQL_DIR);
        let result = loader.read_query("not_found.graphql");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_read_file_with_invalid_extension() {
        let loader = GraphQLDLoader::new(&TEST_GQL_DIR);
        let result = loader.read_query("wrong_file.txt");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
    }
}
