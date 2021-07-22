#!/usr/sbin/dtrace -s

#pragma D option quiet

burst*:::get-start
{
	follow[arg0] = timestamp;
}

burst*:::get-done
/follow[arg0] != 0/
{
	/*
	 * Uncomment if you wish to see how long every request lifetime took.
	 * printf("%d/%d took %d nsecs to finish request lifetime\n",
	 *    pid, tid, timestamp - follow[arg0]);
	 */
	@totalrqs["total requests"] = count();
	@avgtime["average request lifetime"] = avg(timestamp - follow[arg0]);
	@maxtime["max request lifetime"] = max(timestamp - follow[arg0]);
	@mintime["min request lifetime"] = min(timestamp - follow[arg0]);
	@histogram["request lifetimes visualisation"] = quantize(timestamp - follow[arg0]);
	follow[arg0] = 0;
}

tick-10s {
	printf("Summary of all GET request round trips taken in ten seconds represented in nanoseconds:");
    exit(0); 
}