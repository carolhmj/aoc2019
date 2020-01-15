(ns day14.core
  (:gen-class)
  (:require [clojure.string :as str]))

(defn read-input [file-name] 
  (str/split-lines (slurp file-name)))

(defn reagents2pairs [reagents]
  "Transforms a list of reagents in the form
['3 A' '4 B' '5 C'] into [[3 A] [4 B] [5 C]]"
  (map (fn [r] 
         (let [[qty chem] (str/split r #" ")]
           (list (read-string qty) chem))) reagents))

(defn line2record [line]
  (let [[reagents reactor] (str/split line #"=>")]
    (let [reagent-text-list (map str/trim (str/split reagents #","))]
      (let [[qty elem] (str/split (str/trim reactor) #" ")]
        [elem {:qty (read-string qty) :react (reagents2pairs reagent-text-list)}]))))

(defn input2records [input]
  (let [lines (read-input input)]
    (apply (partial assoc nil) (mapcat line2record lines))))

(defn calc-rem
  [used available chem]
  (list (- available used) chem))

(declare required-ore)

(defn ore-reduction 
  [ratio records {ore-acc :qty rem-acc :rem} [qty-elem elem-val]]
  (let
   [{req-elem-qty :qty req-elem-rem :rem} 
    (required-ore elem-val (*' ratio qty-elem) rem-acc records)]
    {:qty (+' req-elem-qty ore-acc) 
     :rem (into rem-acc req-elem-rem)}))

(defn required-ore [chem amount-required available records]
"Returns the amount of ore necessary to produce qty amount of
chemical chem with the available materials"
  (if (= chem "ORE") 
    {:qty amount-required :rem {}} 
    (do (let [{produced-by-reaction :qty elems :react} (get records chem)
              available-chem (get available chem 0)
              used-available (if (> available-chem amount-required) amount-required available-chem)
              new-available (assoc available chem (- available-chem used-available))
              needed (- amount-required used-available)
              ratio (quot needed produced-by-reaction)
              rest (rem needed produced-by-reaction)
              corrected-ratio (if (> rest 0) (+ ratio 1) ratio)
              corrected-rest (- (* corrected-ratio produced-by-reaction) needed)]
          (let [{ore-needed :qty remainder :rem} 
                (reduce (partial ore-reduction corrected-ratio records) {:qty 0 :rem new-available} elems)]
            {:qty ore-needed 
             :rem (assoc remainder chem (+ corrected-rest (get remainder chem 0)))})))))

(defn day14-1
  [input]
  (required-ore "FUEL" 1 {} (input2records input)))

(defn bin-search
  "Do a binary search on vector v for value x"
  [x v]
  (loop [lower 0
         upper (.length v)]
    (let [mid (Math/floorDiv (+ upper lower) 2)
          midValue (nth v mid)
          lengthInterval (- upper lower)] 
      (do (if (= midValue x) 
            mid
            (if (<= lengthInterval 1) 
              -1
              (if (< x midValue)
                (recur lower mid)
                (recur (+ mid 1) upper))))))))

(defn bin-search2
  "Do a binary search on vector v for value x"
  [x v]
  (loop [lower 0
         upper (.length v)]
    (if (>= lower upper) -1 
        (let [mid (Math/floorDiv (+ upper lower) 2)
              midValue (nth v mid)]
          (cond
            (= x midValue) mid
            (< x midValue) (recur lower mid)
            :else          (recur (+ mid 1) upper))))))

(defn day14-2
  "Search for the amount of fuel that will be necessary to
consume the amount of ore ore-amount"
  [input ore-amount initial-lower initial-upper]
  (let [records (input2records input)]
    (loop [lower initial-lower
           upper initial-upper
           prev-mid-smaller 0]
      (if (>= lower upper) prev-mid-smaller
        (let [mid (Math/floorDiv (+ upper lower) 2)
              required-ore (:qty (required-ore "FUEL" mid {} records))]
          ; (prn "lower" lower "mid" mid "upper" upper "required-ore" required-ore "prev-mid-smaller" prev-mid-smaller)
          (cond
            (= ore-amount required-ore) mid
            (< ore-amount required-ore) (recur lower mid prev-mid-smaller)
            :else                       (recur (+ mid 1) upper mid)))))))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (def input "input.txt")
  (println "Amount of ORE needed for 1 FUEL" (:qty (day14-1 input)))
  (println "Amount of FUEL produced with 1B ORE" (day14-2 input 1000000000000 400000 6000000)))
