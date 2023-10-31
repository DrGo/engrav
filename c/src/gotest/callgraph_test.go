package abc_test

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"testing"
)

type void struct{}

var yes void
var funcNames = map[string]void{
	"abort":                            yes,
	"abs":                              yes,
	"access":                           yes,
	"acos":                             yes,
	"alloca":                           yes,
	"asin":                             yes,
	"atan":                             yes,
	"atexit":                           yes,
	"atof":                             yes,
	"atoi":                             yes,
	"bsearch":                          yes,
	"calloc":                           yes,
	"ceil":                             yes,
	"chdir":                            yes,
	"chmod":                            yes,
	"clock":                            yes,
	"close":                            yes,
	"closedir":                         yes,
	"cos":                              yes,
	"difftime":                         yes,
	"dup":                              yes,
	"dup2":                             yes,
	"execve":                           yes,
	"exit":                             yes,
	"exp":                              yes,
	"fabs":                             yes,
	"fclose":                           yes,
	"fgetc":                            yes,
	"fgets":                            yes,
	"floor":                            yes,
	"fmod":                             yes,
	"fopen":                            yes,
	"fork":                             yes,
	"fprintf":                          yes,
	"fputc":                            yes,
	"fread":                            yes,
	"free":                             yes,
	"fscanf":                           yes,
	"fstat":                            yes,
	"fwrite":                           yes,
	"getcwd":                           yes,
	"getenv":                           yes,
	"getpid":                           yes,
	"gettimeofday":                     yes,
	"gmtime":                           yes,
	"isalnum":                          yes,
	"isalpha":                          yes,
	"isdigit":                          yes,
	"islower":                          yes,
	"isupper":                          yes,
	"kill":                             yes,
	"labs":                             yes,
	"localtime":                        yes,
	"log":                              yes,
	"log10":                            yes,
	"longjmp":                          yes,
	"lseek":                            yes,
	"malloc":                           yes,
	"memcpy":                           yes,
	"memmove":                           yes,
	"memset":                           yes,
	"mkdir":                            yes,
	"mktime":                           yes,
	"open":                             yes,
	"opendir":                          yes,
	"perror":                           yes,
	"pipe":                             yes,
	"pow":                              yes,
	"printf":                           yes,
	"pthread_attr_destroy":             yes,
	"pthread_attr_getdetachstate":      yes,
	"pthread_attr_getguardsize":        yes,
	"pthread_attr_getinheritsched":     yes,
	"pthread_attr_getschedparam":       yes,
	"pthread_attr_getschedpolicy":      yes,
	"pthread_attr_getscope":            yes,
	"pthread_attr_getstacksize":        yes,
	"pthread_attr_init":                yes,
	"pthread_attr_setdetachstate":      yes,
	"pthread_attr_setguardsize":        yes,
	"pthread_attr_setinheritsched":     yes,
	"pthread_attr_setschedparam":       yes,
	"pthread_attr_setschedpolicy":      yes,
	"pthread_attr_setscope":            yes,
	"pthread_attr_setstack":            yes,
	"pthread_attr_setstacksize":        yes,
	"pthread_cancel":                   yes,
	"pthread_cleanup_pop":              yes,
	"pthread_cleanup_push":             yes,
	"pthread_cond_destroy":             yes,
	"pthread_cond_init":                yes,
	"pthread_cond_signal":              yes,
	"pthread_cond_timedwait":           yes,
	"pthread_cond_wait":                yes,
	"pthread_create":                   yes,
	"pthread_detach":                   yes,
	"pthread_equal":                    yes,
	"pthread_exit":                     yes,
	"pthread_getattr_np":               yes,
	"pthread_getconcurrency":           yes,
	"pthread_getcpuclockid":            yes,
	"pthread_getschedparam":            yes,
	"pthread_getspecific":              yes,
	"pthread_join":                     yes,
	"pthread_key_create":               yes,
	"pthread_key_delete":               yes,
	"pthread_kill":                     yes,
	"pthread_mutex_destroy":            yes,
	"pthread_mutex_init":               yes,
	"pthread_mutex_lock":               yes,
	"pthread_mutex_trylock":            yes,
	"pthread_mutex_unlock":             yes,
	"pthread_mutexattr_destroy":        yes,
	"pthread_mutexattr_getprotocol":    yes,
	"pthread_mutexattr_getpshared":     yes,
	"pthread_mutexattr_gettype":        yes,
	"pthread_mutexattr_init":           yes,
	"pthread_mutexattr_setprioceiling": yes,
	"pthread_mutexattr_setprotocol":    yes,
	"pthread_mutexattr_setpshared":     yes,
	"pthread_mutexattr_settype":        yes,
	"pthread_once":                     yes,
	"pthread_rwlock_destroy":           yes,
	"pthread_rwlock_init":              yes,
	"pthread_rwlock_rdlock":            yes,
	"pthread_rwlock_tryrdlock":         yes,
	"pthread_rwlock_trywrlock":         yes,
	"pthread_rwlock_unlock":            yes,
	"pthread_rwlock_wrlock":            yes,
	"pthread_self":                     yes,
	"pthread_setcancelstate":           yes,
	"pthread_setcanceltype":            yes,
	"pthread_setconcurrency":           yes,
	"pthread_setschedparam":            yes,
	"pthread_setspecific":              yes,
	"pthread_testcancel":               yes,
	"qsort":                            yes,
	"rand":                             yes,
	"read":                             yes,
	"readdir":                          yes,
	"realloc":                          yes,
	"remove":                           yes,
	"rename":                           yes,
	"rewinddir":                        yes,
	"rmdir":                            yes,
	"setenv":                           yes,
	"setjmp":                           yes,
	"sigaction":                        yes,
	"signal":                           yes,
	"sin":                              yes,
	"snprintf":                         yes,
	"sprintf":                          yes,
	"sqrt":                             yes,
	"srand":                            yes,
	"sscanf":                           yes,
	"stat":                             yes,
	"strcat":                           yes,
	"strdup":                           yes,
	"strrchr":                           yes,
	"strchr":                           yes,
	"strcmp":                           yes,
	"strcpy":                           yes,
	"strftime":                         yes,
	"strlen":                           yes,
	"strncat":                          yes,
	"strncmp":                          yes,
	"strstr":                           yes,
	"strtol":                           yes,
	"strtoul":                          yes,
	"sysconf":                          yes,
	"system":                           yes,
	"tan":                              yes,
	"time":                             yes,
	"times":                            yes,
	"tolower":                          yes,
	"toupper":                          yes,
	"unsetenv":                         yes,
	"utime":                            yes,
	"wait":                             yes,
	"waitpid":                          yes,
	"write":                            yes,
	"vfprintf":                            yes,
	"vsnprintf":                            yes,
	"va_end":                            yes,
	"va_start":                            yes,
	"va_list":                            yes,
	"fputs":                            yes,
	"fseek":                            yes,
	"ftell":                            yes,
	"rewind":                            yes,
	"mmap":                            yes,
	"munmap":                            yes,
	"strerror":                            yes,
	"isspace":                            yes,
	"regcomp":                            yes,
	"regexec":                            yes,
	"regfree":                            yes,
	"strtod":                            yes,
	"strncpy":                            yes,
	"fileno":                            yes,
}

const (
	graphFileName = "callgraph.txt"
	graphCleanedFileName = "cleangraph.txt"
)


func printLines(filename string, values []string) error {
    f, err := os.Create(filename)
    if err != nil {
        return err
    }
    defer f.Close()
    for _, value := range values {
       fmt.Fprintln(f, value)  // print values to f, one per line
    }
    return nil
}

func TestCleanGraph(t *testing.T) {
	file, err := os.Open(graphFileName)
	if err != nil {
		t.Fatal(err)
	}
	defer file.Close()

	sc := bufio.NewScanner(file)
	lines := make([]string, 0, 2000)

	for sc.Scan() {
		s := strings.TrimSpace(sc.Text())
		lparen := strings.Index(s, "(")
		if lparen == -1 {
			lines = append(lines, sc.Text())
			continue
		}
		fn := s[:lparen]
		if _, ok := funcNames[fn]; ok {
			continue //skip
		}
		lines = append(lines, sc.Text())
	}

	if err := sc.Err(); err != nil {
		t.Fatal(err)
	}
	
	if err := printLines(graphCleanedFileName, lines); err != nil {
		t.Fatal(err)
	}
}
