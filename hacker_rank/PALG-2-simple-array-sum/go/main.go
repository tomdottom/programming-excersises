package main
import (
  "fmt"
  "bufio"
  "os"
  "strconv"
  "strings"
)

func sum(n int64, numbers []int64) int64{
    var res int64
    res = 0
    for i := 0; i < int(n); i++ {
        res = res + numbers[i]
    }
    return res
}

func main() {
    scanner := bufio.NewScanner(os.Stdin)

    scanner.Scan()
    _n := scanner.Text()
    n, err := strconv.ParseInt(strings.TrimRight(_n, "\n"), 10, 64)
    checkError(err)

    scanner.Scan()
    _numbers := scanner.Text()
    _numbersAr := strings.Split(_numbers, " ")
    var ar []int64
    for arrItr := 0; arrItr < int(n); arrItr++ {
        tmp_n, _ := strconv.ParseInt(strings.TrimRight(_numbersAr[arrItr], "\n"), 10, 64)
        ar = append(ar, tmp_n)
    }

    res := sum(n, ar)

    fmt.Println(res)
}

func checkError(err error) {
    if err != nil {
        panic(err)
    }
}