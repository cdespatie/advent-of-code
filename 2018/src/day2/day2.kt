package day2

import java.io.File

fun day2() {
    val inputLines = File("./src/day2/input.txt").readLines()
    println(part1(inputLines))
    println(part2(inputLines))
}

fun part1(input: Collection<String>): Int {
    var twoCount = 0
    var threeCount = 0

    input.forEach { line ->
        val map = HashMap<Char, Int>()

        line.forEach{ char ->
            map[char] = (map[char] ?: 0) + 1
        }

        if (map.any { x -> x.value == 2 }) {
            twoCount += 1
        }
        if (map.any { x -> x.value == 3 }) {
            threeCount += 1
        }
    }

    return twoCount * threeCount
}

fun part2(input: Collection<String>): String? {
    for (i in 0 until input.count()) {
        val current = input.elementAt(i)

        for (j in 0 until input.first().length) {
            for (x in i + 1 until input.count()) {
                if (current.removeRange(j, j + 1) == input.elementAt(x).removeRange(j, j + 1)) {
                    return current.removeRange(j, j + 1)
                }
            }
        }
    }

    return null
}
