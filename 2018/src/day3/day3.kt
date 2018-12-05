package day3

import java.io.File

fun day3() {
    val inputLines = File("./src/day3/input.txt").readLines()

    val parsed = inputLines.map { parse(it) }
    part1(parsed)
}

fun part1(instructions: Collection<Instruction>) {
    val array = Array(1001) { Array(1001) { "-" } }

    for (instruction in instructions) {
        for (i in instruction.x..(instruction.x + instruction.width)) {
            for (j in instruction.y..(instruction.y + instruction.height)) {
                array[i][j] = if (array[i][j] != "-") "#" else "x"
            }
        }
    }

    val overlaps = array.map { x -> x.count { y -> y == "#"} }.sum()

//    printMap(array)
    println(overlaps)
}

fun parse(input: String): Instruction {
    val regex = Regex("([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)")
    val match = regex.find(input.trim())
    val groupVals = match?.groupValues

    return Instruction(groupVals?.get(1)?.toInt() ?: 0, groupVals?.get(2)?.toInt() ?: 0,
    groupVals?.get(3)?.toInt() ?: 0, groupVals?.get(4)?.toInt() ?: 0,
        groupVals?.get(5)?.toInt() ?: 0)
}

fun printMap(array: Array<Array<String>>) {
    for (x in array) {
        for (y in x) {
            print(y)
        }
        println("")
    }
}

data class Instruction(val id: Int, val x: Int, val y: Int, val width: Int, val height: Int)