# Тестируем лабы
*Они забрали у нас тесты, мы сделали свои...*

## Что это такое
Это довольно простая программа, которая умеет генерировать
случайные тесты в большом количестве и скармливать их вашей проге.

## Как это работает
Штука генерирует сначала решенную матрицу, а потом
с помощью эквивалентных преобразований уродует ее до неузнаваемости.

## Как этим пользоваться
К сожалению вам придется сделать некоторые изменения в своей программе.
Так как создавать каждый раз файл и каждый раз перезапускать прогу кажется чем-то
очень медленным в данной утилите используется stdin/stdout.

### Как переделать свой код чтобы тестировалка заработала с ним
Это в целом очень просто.

Вот допустим у вас вот такой простой код:
```c
FILE *input = fopen("input.txt", "r");
FILE *output = fopen("output.txt", "w");
// Some error handling

solve(input, output) // Тут мы решаем уравнение и выводим результат
```
Наша задача сделать так, чтобы наша программа читала из std i/o 
и принимала более одного результата.  
Тестировалка передает в программу одну переменную среды под названием `TEST`. 
Воспользуемся же этим и слегка подправим наш код:
```c
FILE *input;
FILE *output;
if (!getenv("TEST")) {
    input = fopen(args[1], "r");
    output = fopen(args[2], "w");
} else {
    input = stdin;
    output = stdout;
}
// Some error handling

do {
solve(input, output); // Тут мы решаем уравнение и выводим результат
fflush(output); // Это важная штука
} while (getenv("TEST"))
```

### Как запустить тестирование
1. Скачиваем вот [отсюда](https://github.com/JustAGod1/c-labs-tester/releases) последний релиз
2. Открываем консоль и вводим `<tester> --executable <ваша скомпилированная в ехешник лаба> --lab slae`
3. Для более подробного описания можно ввести `<tester> --help` 

