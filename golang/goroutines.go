package main

import "sync" // mutex

/*
	Goroutines are more likely sth between async execution but includes os multithreading as well
	* - main () is also goroutine
*/

func someEmtpyFunc() {}

func spawnGoroutines() {
	go someEmtpyFunc()

	go func(x int) { println(x) }(5)
}

/*
	Channel is a sync method
*/

func channelOperations() {
	var intChn chan int // <-- init

	var data int
	go func() { intChn <- 5 }()
	go func() { data = <-intChn }()
	println(data)

	/*
		If channel is empty, goroutine-receiver is blocked.
		If channel is full, goroutine-sender is blocked.
	*/
}

func nonBuffChannels() {
	var floatChn chan float32 = make(chan float32)
	strChn := make(chan string)

	if strChn == nil || floatChn != nil {
		/* Empty */
	}
}

func buffChannels() {
	var floatChn chan float32 = make(chan float32, 5) // <-- can store 5 values
	strChn := make(chan string, 3)

	println(cap(floatChn))
	println(len(strChn))
}

func oneWayChannelsInit() {
	var inCh chan<- int
	var outCh <-chan int

	println(cap(inCh), cap(outCh))
}

func createAndReturnChannel() (intChn chan int) {
	intChn = make(chan int)
	x := 5

	go func() { intChn <- x }()
	return
}

func stateCheckAndClose() {
	var intChn chan int
	go func() { intChn <- 5 }()

	if value, isOpened := <-intChn; isOpened {
		close(intChn)
	} else {
		println(value)
	}

	<-intChn // is the synonim to "wait for channel to close"
}

func mutexUsage() {
	mutex := sync.Mutex{}

	var numbers []int = []int{1, 2, 3}

	var counter = 0
	write := func() {
		mutex.Lock()
		numbers = append(numbers, counter)
		counter++
		mutex.Unlock()
	}

	go write()
	go write()
}

func WaitGroupUsage() {
	var wg sync.WaitGroup
	wg.Add(2) // 2 goroutines

	var counter = 0
	job := func() {
		defer wg.Done() // notify wg that goroutine is done
		counter++
	}

	go job()
	go job()

	wg.Wait() // wait till all goroutines in group are done
	if counter != 2 {
		panic("epic fail")
	}
}
