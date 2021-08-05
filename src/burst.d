provider burst {
	probe get__start(uint64_t);
	probe get__done(uint64_t);
	probe post__start(uint64_t);
	probe post__done(uint64_t);
	probe put__start(uint64_t);
	probe put__done(uint64_t);
	probe patch__start(uint64_t);
	probe patch__done(uint64_t);
	probe requests__start(uint64_t);
	probe requests__done(uint64_t);
};