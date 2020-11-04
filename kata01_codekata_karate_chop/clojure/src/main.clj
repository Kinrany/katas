(ns main)

(defn chop [x, arr]
  (let [left (atom 0)
        right (atom (count arr))
        slice_length (fn [] (- @right @left))
        found (atom false)]
    (while (and (> (slice_length) 1)
                (not @found))
      (let [center (/ (+ @left @right) 2)]
        (case (compare (nth arr center) x)
          -1 (reset! left center)
          1 (reset! right center)
          0 (do
              (reset! found true)
              (reset! left center)))))
    @left))

(defn chop_recursive [x, arr]
  (if (= (count arr) 1) 0
    (let [center (/ (+ 0 (count arr)) 2)]
      (case (compare (nth arr center) x)
        -1 (+ center (chop_recursive x (subvec arr center)))
        1 (chop_recursive x (subvec arr 0 center))
        0 center))))

(defn main [opts]
  (printf "chop(5, [1, 3, 5, 7]): %s%n" (chop_recursive 5 [1 3 5 7])))
