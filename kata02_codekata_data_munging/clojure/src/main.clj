(ns main
  (:require [clojure.string :refer [split trim]]
            [clojure.tools.cli :refer [parse-opts]]))

(def cli_options
  [["-f" "--file FILE"]])

(defn parse_day [line]
  (try
    ; trim before splitting: normally leading whitespace
    ; will turn into a leading empty string
    (let [words (split (trim line) #"\s+")
          day_number (Integer/parseInt (nth words 0))
          max_temperature (Integer/parseInt (nth words 1))
          min_temperature (Integer/parseInt (nth words 2))]
      { :day_number day_number
        :max_temperature max_temperature
        :min_temperature min_temperature})
    (catch Exception e nil)))

(defn temperature_spread [day]
  (let [{:keys [max_temperature min_temperature]} day]
    (- max_temperature min_temperature)))

(def opts (parse-opts *command-line-args* cli_options))
(def file (slurp ((opts :options) :file)))
(def lines (clojure.string/split-lines file))
(def days (->> lines
  (map parse_day)
  (remove nil?)))
(def day_with_smallest_spread (apply min-key temperature_spread days))

(let [day day_with_smallest_spread]
  (printf "Day %s has the smallest temperature spread of %s.%n"
    (day :day_number)
    (temperature_spread day)))
