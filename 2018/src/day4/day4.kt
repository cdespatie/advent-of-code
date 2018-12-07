package day4

import java.io.File
import java.time.LocalDate
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter
import java.time.temporal.ChronoField
import java.time.temporal.ChronoUnit

fun day4() {
    val inputLines = File("./src/day4/input.txt").readLines()
    val events = inputLines.map { parse(it) }.sortedBy { it.timestamp }

    part1(events)
}

fun part1(input: Collection<Event>) {
    val guardMap = HashMap<String, MutableCollection<TimeRange>>()
    var currentGuard = ""
    var currentTimestamp = LocalDateTime.now()

    for (event in input) {
        when {
            event.value.startsWith("Guard") -> {
                currentGuard = getGuardId(event.value)
            }
            event.value.startsWith("falls") -> {
                currentTimestamp = event.timestamp
            }
            else -> {
                if (guardMap.containsKey(currentGuard)) {
                    guardMap[currentGuard]?.add(TimeRange(currentTimestamp, event.timestamp))
                }
                else {
                    guardMap[currentGuard] = mutableListOf(TimeRange(currentTimestamp, event.timestamp))
                }
            }
        }
    }

    val biggest = guardMap.mapValues { x -> x.value.sumBy { y -> y.startTime.until(y.endTime, ChronoUnit.SECONDS).toInt() } }
        .toList().sortedBy { (_, x) -> x }.last()

    val overlapMinute = findOverlap(guardMap[biggest.first] ?: emptyList())
    println(overlapMinute.first * biggest.first.toInt())

    var highest = Pair(0, 0)
    var highestGuard = ""

    for (guard in guardMap) {
        val overlap = findOverlap(guard.value)
        if (overlap.second > highest.second) {
            highest = overlap
            highestGuard = guard.key
        }
    }

    println("$highestGuard $highest")
    println(highestGuard.toInt() * highest.first)
}

fun findOverlap(input: Collection<TimeRange>): Pair<Int, Int> {
    val dayMap = HashMap<Int, Int>()
    val groupedByDate = input.groupBy { x -> x.startTime.toLocalDate() }

    for (day in groupedByDate) {
        for (min in 0 until 60) {
            if (day.value.any { y -> y.startTime.minute <= min && y.endTime.minute > min }) {
                dayMap[min] = (dayMap[min] ?: 0) + 1
            }
        }
    }

    return (dayMap.toList().sortedBy { (_, x) -> x }).last()
}

fun getGuardId(input: String): String {
    val regex = Regex("#([0-9]+)")
    val match = regex.find(input.trim())

    return match?.groupValues?.get(1) ?: ""
}

fun parse(input: String): Event {
    val dateFormat = "yyyy-MM-dd HH:mm"

    val regex = Regex("\\[(.*)\\] (.*)")
    val match = regex.find(input.trim())
    val groupVals = match?.groupValues

    return Event(LocalDateTime.parse(groupVals?.get(1) ?: "", DateTimeFormatter.ofPattern(dateFormat)), groupVals?.get(2) ?: "")
}

data class Event(val timestamp: LocalDateTime, val value: String)
data class TimeRange(val startTime: LocalDateTime, val endTime: LocalDateTime)
