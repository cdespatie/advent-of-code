package day3

import java.io.File

fun day3() {
    val inputLines = File("./src/day3/input.txt").readLines()

    val parsed = inputLines.map { parse(it) }
    bothParts(parsed)
}

fun bothParts(instructions: Collection<Instruction>) {
    val array = Array(1001) { Array(1001) { "-" } }

    for (instruction in instructions) {
        for (i in instruction.x until (instruction.x + instruction.width)) {
            for (j in instruction.y until (instruction.y + instruction.height)) {
                if (array[i][j] != "-" && array[i][j] != "#") {
                    markInstructionIncomplete(instructions, array[i][j])
                    markInstructionIncomplete(instructions, instruction.id.toString())
                    array[i][j] = "#"
                }
                else {
                    array[i][j] = instruction.id.toString()
                }
            }
        }
    }

    val overlaps = array.map { x -> x.count { y -> y == "#"} }.sum()
    val complete = instructions.filter { x -> x.isComplete }

    println(overlaps)
    println(complete)
}

fun markInstructionIncomplete(instructions: Collection<Instruction>, id: String) {
    val instruction = instructions.find { x -> x.id == id.toInt() }
    instruction?.isComplete = false
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

data class Instruction(val id: Int, val x: Int, val y: Int, val width: Int, val height: Int) {
    var isComplete: Boolean = true
}