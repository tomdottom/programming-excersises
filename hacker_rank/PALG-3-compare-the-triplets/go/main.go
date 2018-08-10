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

func get_triplet(scanner *bufio.Scanner) []int64 {
	scanner.Scan()
	text_triplet := strings.Split(scanner.Text(), " ")
	var triplet []int64
	for _, val := range text_triplet {
		tmp, _ := strconv.ParseInt(val, 10, 64)
		triplet = append(triplet, tmp)
	}
	return triplet
}

func calculate_points(a, b []int64) (int64, int64) {
	var a_score int64
	var b_score int64
	for i, _ := range a {
		if a[i] > b[i] {
			a_score++
		} else if a[i] < b[i] {
			b_score++
		}
	}
	return a_score, b_score
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	a := get_triplet(scanner)
	b := get_triplet(scanner)

	fmt.Println(calculate_points(a, b))
}

func checkError(err error) {
	if err != nil {
		panic(err)
	}
}