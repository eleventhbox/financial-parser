# cli-converter

Конвертирует транзакции из одного формата в другой (CSV, TEXT, BINARY).

### Использование
cli-converter --input <входной_файл> --input-format <формат_ввода> --output <выходной_файл> --output-format <формат_вывода>

### Примеры
#### Конвертировать CSV в бинарный формат
cli-converter --input input.csv --input-format csv --output output.bin --output-format binary
#### Конвертировать бинарный в текст
cli-converter --input input.bin --input-format binary --output output.txt --output-format text