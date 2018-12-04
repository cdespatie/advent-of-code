package day3

import java.io.File

fun day3() {
    val inputLines = File("./src/day3/input.txt").readLines()

    for (line in inputLines) {
        println(parse(line))
    }
}

fun parse(input: String): Instruction {
    val regex = Regex("([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)")
    val match = regex.find(input.trim())
    val groupVals = match?.groupValues

    return Instruction(groupVals?.get(1)?.toInt() ?: 0, groupVals?.get(2)?.toInt() ?: 0,
    groupVals?.get(3)?.toInt() ?: 0, groupVals?.get(4)?.toInt() ?: 0,
        groupVals?.get(5)?.toInt() ?: 0)
}

data class Instruction(val id: Int, val x: Int, val y: Int, val width: Int, val height: Int)