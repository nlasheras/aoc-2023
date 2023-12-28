(declare-const px Int)
(declare-const py Int)
(declare-const pz Int)
(declare-const vx Int)
(declare-const vy Int)
(declare-const vz Int)

(declare-const t1 Int)
(assert (= (+ 19 (* -2 t1)) (+ px (* vx t1))))
(assert (= (+ 13 (* 1 t1)) (+ py (* vy t1))))
(assert (= (+ 30 (* -2 t1)) (+ pz (* vz t1))))

(declare-const t2 Int)
(assert (= (+ 18 (* -1 t2)) (+ px (* vx t2))))
(assert (= (+ 19 (* -1 t2)) (+ py (* vy t2))))
(assert (= (+ 22 (* -2 t2)) (+ pz (* vz t2))))

(declare-const t3 Int)
(assert (= (+ 20 (* -2 t3)) (+ px (* vx t3))))
(assert (= (+ 25 (* -2 t3)) (+ py (* vy t3))))
(assert (= (+ 34 (* -4 t3)) (+ pz (* vz t3))))

(declare-const t4 Int)
(assert (= (+ 12 (* -1 t4)) (+ px (* vx t4))))
(assert (= (+ 31 (* -2 t4)) (+ py (* vy t4))))
(assert (= (+ 28 (* -1 t4)) (+ pz (* vz t4))))

(declare-const t5 Int)
(assert (= (+ 20 (* 1 t5)) (+ px (* vx t5))))
(assert (= (+ 19 (* -5 t5)) (+ py (* vy t5))))
(assert (= (+ 15 (* -3 t5)) (+ pz (* vz t5))))

(declare-const PART2 Int)
(assert (= PART2 (+ px (+ py pz))))

(check-sat)
(get-model)