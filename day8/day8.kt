import java.io.File;

fun main(args: Array<String>) {
    val size = 25 * 6;
    val layers : List<String> = File("input.txt")
        .readText().chunked(size);

    // println(layers);

    val layersOrderedByNumberOfZeroes = 
        layers.sortedBy { layer -> layer.count {c -> c == '0'}};

    // println(layersOrderedByNumberOfZeroes);

    val layerWithLeastZeroes = layersOrderedByNumberOfZeroes[0];

    val numberOfOnes = layerWithLeastZeroes.count {c -> c == '1'};  
    val numberOfTwos = layerWithLeastZeroes.count {c -> c == '2'};

    println("""Number of ones is ${numberOfOnes} and number of twos
    is ${numberOfTwos} the two multiplied are ${numberOfOnes *
    numberOfTwos}""");

    val finalImage = MutableList<Char>(size) { '2' };

    for (i in 0 until size) {
        finalImage[i] = layers.map{ it[i] }.reduce { currentVisiblePixel, pixel -> 
            if (currentVisiblePixel == '2') pixel else currentVisiblePixel };
    }

    val structuredImage = finalImage.chunked(25) {it.joinToString("")}
        .joinToString(separator = "\n");

    println("The message is \n${structuredImage}");  
}