(ns day10.core
  (:gen-class)
  (:require [clojure.string :as str] 
            [clojure.pprint :as pretty]))

(defn parse-map-into-objects
  [map]
   (mapcat
    (fn [row item]
      (map-indexed
       (fn [col obj]
         [col row obj]) 
       item))
    (range) 
    map))

(defn filter-out-non-asteroids
  [map]
  (filter 
   (fn [[_ _ obj]] 
     (= obj \#)) 
   map))

(defn read-input
  [file-name]
  (filter-out-non-asteroids
   (parse-map-into-objects
    (str/split-lines
     (slurp file-name)))))

(def input (read-input "test.txt"))

(defn angle-between
  [a b]
  (let [[xa ya] a
        [xb yb] b]
    (Math/atan2 
     (- yb ya) 
     (- xb xa))))

(defn distance-squared
  [a b]
  (let [[xa ya] a
        [xb yb] b]
    (+ (Math/pow (- xb xa) 2)
       (Math/pow (- yb ya) 2))))

(defn fill-observation-entry
  [base asteroid]
  {:angle (angle-between base asteroid)
   :distance (distance-squared base asteroid)
   :coords asteroid})

(defn run-observations-for-base
  [base asteroid-list]
   (let [[bx by] base]
     (map
      #(fill-observation-entry base %)
      (filter (fn [[ax ay]] (or (not= ax bx) (not= ay by))) asteroid-list))))

(defn run-observations-for-all-bases
  [asteroid-list]
  (map 
   #(assoc {} :base % :observations (run-observations-for-base % asteroid-list))
   asteroid-list))

(defn count-by-angle
  [list-of-observations]
  (count 
   (group-by 
    (fn [{angle :angle}] angle) 
    list-of-observations)))

(defn get-highest-observation-count
  [list-of-observations]
  (apply max-key :angle-count
   (map (fn [dict] 
          (assoc dict :angle-count (count-by-angle (:observations dict)))) 
        list-of-observations)))

(defn day10-1
  [input-file]
  (get-highest-observation-count
   (run-observations-for-all-bases
    (read-input input-file))))

(def base-test (day10-1 "test.txt"))
(def observations-base (:observations base-test))
(def new-base (day10-1 "test5.txt"))
(def new-observations (:observations new-base))

(defn fourth_quadrant?
  [angle]
  (and (<= (- Math/PI) angle)
       (< angle (/ (- Math/PI) 2))))

(defn compare-by-ray
  "Order two observations, with the one that will be reached by the laser first in 
the start"
  [a b]
  (case (map fourth_quadrant? [a b])
    ((true true)) (compare (Math/abs b) (Math/abs a))
    ((false false)) (compare a b)
    ((true false)) 1
    ((false true)) -1))

(defn group-and-sort-observations
  [observations]  
 (into 
  (sorted-map-by compare-by-ray) 
  (group-by :angle (sort-by :distance observations))))

(def grouped-obs (group-and-sort-observations observations-base))
(def grouped-new-obs (group-and-sort-observations new-observations))

(defn map-function-on-map-vals [m f]
  (zipmap (keys m) (map f (vals m))))

(defn take-first-of-each-map
  [map]
  (filter 
   (fn [[_ v]] (seq v)) 
   (map-function-on-map-vals map #(rest %))))

(defn destroy-asteroids-until
  [stop-at observations]
  (loop [remaining-observations observations next-value 0]
    (let [next-to-see (vals remaining-observations) 
          next-next-val (+ next-value (count next-to-see))]
      ; (pretty/pprint next-to-see)
      ; (pretty/pprint next-next-val)
      (if (>= next-next-val stop-at)
        (let [res (first (get (into [] next-to-see) (- stop-at next-value 1)))]
          ; (pretty/pprint res)
          res)
        (recur (take-first-of-each-map remaining-observations) next-next-val)))))

(def asteroid200new (destroy-asteroids-until 200 grouped-new-obs))

(defn day10-2
  [new-base-localization]
  (destroy-asteroids-until 200
                           (group-and-sort-observations (:observations new-base-localization))))

(defn -main
  [& args]
  (let [new-base (day10-1 "input.txt") asteroid200 (day10-2 new-base)]
    (println (str "The asteroid with most
observations is... " (:base new-base) " with " (:angle-count new-base) " observations"))
    (println (str "The 200th destroyed asteroid is..." asteroid200))))
