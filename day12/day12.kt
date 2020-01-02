import kotlin.math.*
import java.io.File

data class Moon(val position : IntArray, val velocity : IntArray = intArrayOf(0,0,0))
data class IndexAndValue<T>(val index : Int, val value : T)

fun <T, U> cartesianProductIndexed(c1: Collection<T>, c2: Collection<U>): List<Pair<IndexAndValue<T>, IndexAndValue<U>>> {
    return c1.mapIndexed { lhsIdx, lhsElem -> c2.mapIndexed { rhsIdx, rhsElem -> IndexAndValue(lhsIdx, lhsElem) to IndexAndValue(rhsIdx, rhsElem) } }.flatten()
}

fun sumArrays(a : IntArray, b : IntArray) : IntArray {
    return a.zip(b).map {(x, y) -> x + y}.toIntArray()
}

fun sumAbs(a : IntArray) : Int {
    return a.map { abs(it) }.sum()
}

fun calculateEnergy(moons : List<Moon>) : Int {
    return moons.map { sumAbs(it.position) * sumAbs(it.velocity) }.sum()
}

fun step(moons : List<Moon>) : List<Moon> {
    val accelerations = cartesianProductIndexed(moons, moons).map { (moonA, moonB) -> 
        moonA.index to moonA.value.position.zip(moonB.value.position).map { (posA : Int, posB : Int) ->
            (posB - posA).sign
        }.toIntArray() 
    }.groupBy({ it.first }, { it.second }).map { (k, v) -> k to v.reduce({ acc : IntArray, curr : IntArray -> sumArrays(acc, curr) }) }

    val updatedMoons = accelerations.map { (idx, acceleration) -> 
        val newVelocity = sumArrays(moons[idx].velocity, acceleration)
        val newPosition = sumArrays(moons[idx].position, newVelocity)
        Moon(newPosition, newVelocity) 
    }
    
    return updatedMoons
}

fun readInput(filename : String) : List<Moon> {
    return File(filename).readLines().map {
        val (x, y, z) = """<x=(-?\d+), y=(-?\d+), z=(-?\d+)>""".toRegex().find(it)!!.destructured
        Moon(intArrayOf(x.toInt(), y.toInt(), z.toInt()))
    }
}

fun main() {
//    var moons = listOf(Moon(intArrayOf(-1,0,2)), Moon(intArrayOf(2,-10,-7)), Moon(intArrayOf(4,-8,8)), Moon(intArrayOf(3,5,-1)))
//    var moons = listOf(Moon(intArrayOf(-8,-10,0)), Moon(intArrayOf(5,5,10)), Moon(intArrayOf(2,-7,3)), Moon(intArrayOf(9,-8,-3)))

    var moons = readInput("input.txt")
    println("Input moons ${moons}")

    var steps = 1000
    for (i in 1..steps) {
        moons = step(moons)
    }
    println("moons after loop ${moons}")
    val energy = calculateEnergy(moons)
    println("Calculated energy ${energy}")
}