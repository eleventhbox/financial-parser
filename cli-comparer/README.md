# cli-comparer

Сравнивает транзакции из двух файлов, поддерживая разные форматы (CSV, TEXT, BINARY).

### Использование
cli-comparer --file1 <файл1> --format1 <формат1> --file2 <файл2> --format2 <формат2>

### Пример
#### Сравнить два файла в разных форматах

cli-comparer --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv

#### Вывод:

The transaction records in 'records_example.bin' and 'records_example.csv' are identical.