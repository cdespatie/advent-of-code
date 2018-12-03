package day1

import java.io.File

fun day1() {
    val inputLines = File("./src/day1/input.txt").readLines()
    println(part1(inputLines))
    println(part2(inputLines))
}

fun part1(inputLines: Collection<String>) : Int {
    return inputLines.map { x -> x.toInt() }.sum()
}

fun part2(inputLines: Collection<String>) : Int {
    var counter = 0
    val set = mutableSetOf(counter)

    while(true) {
        for (line in inputLines) {
            counter += line.toInt()

            if (set.contains(counter)) {
                return counter
            }
            else {
                set.add(counter)
            }
        }
    }
}
