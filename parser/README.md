# financial-parser

Библиотека для парсинга и сериализации финансовых транзакций в различных форматах: CSV, TEXT, BINARY.

### Использование
#### Парсинг
use financial_parser::parser::Parser;

use financial_parser::format::Format;

let mut reader = /* ... */;

let transactions = Parser::parse(&mut reader, Format::Csv)?;

#### Сериализация
use financial_parser::parser::Parser;

use financial_parser::format::Format;

let transactions = /* Vec<Transaction> */;

let mut writer = /* ... */;

Parser::write(&transactions, &mut writer, Format::Binary)?;

### Поддерживаемые форматы
csv: CSV-файл с заголовками

text: Простой текстовый формат

binary: Бинарный формат (bincode)