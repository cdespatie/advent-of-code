package day4

import java.io.File

fun day4() {
    val inputLines = File("./src/day4/input.txt").readLines()
    val events = inputLines.map { parse(it) }.sortedBy { it.timestamp }

    part1(events)
}

fun part1(input: Collection<Event>) {
    val guardMap = HashMap<String, Long>()
    var currentGuard = ""
    var currentTimestamp = 0L

    for (event in input) {
        when {
            event.value.startsWith("Guard") -> {
                currentGuard = getGuardId(event.value)
            }
            event.value.startsWith("falls") -> {
                currentTimestamp = event.timestamp
            }
            else -> {
                guardMap[currentGuard] = guardMap[currentGuard]?.plus((event.timestamp - currentTimestamp)) ?: 0
            }
        }
    }

    val biggest = guardMap.toList().sortedBy { (_, value) -> value }.last()

    println(guardMap)
    println(biggest)
}

fun getGuardId(input: String): String {
    val regex = Regex("#([0-9]+)")
    val match = regex.find(input.trim())

    return match?.groupValues?.get(1) ?: ""
}

fun parse(input: String): Event {
    val regex = Regex("\\[(.*)\\] (.*)")
    val match = regex.find(input.trim())
    val groupVals = match?.groupValues

    return Event(groupVals?.get(1)?.replace("-", "")?.replace(":", "")
        ?.replace(" ", "")?.toLong() ?: 0, groupVals?.get(2) ?: "")
}

data class Event(val timestamp: Long, val value: String)
