package abc_test

import (
	"errors"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"testing"
)

// update golden files if true (Warning: will overwrite golden files if true )
const update = false   
const abcdir = "/Users/drgo/local/code/abc/c/src/"
const testdir = abcdir + "/tests/" //where the abc sample files are stored
const abcexe = abcdir + "/abcm2ps"

// runabc runs abcm2ps and return error and exitCode != 0 if it faield,
// or err and -1 if failed due to other reasons
// or nil and 0 if it worked
func runabc(dir, filename string) (err error, exitCode int) {
	cmd := exec.Command(abcexe, "-g", filename)
	cmd.Dir = dir
	err = cmd.Run()
	if err == nil {
		return nil, 0
	}
	var ee *exec.ExitError
	if errors.As(err, &ee) {
		return err, ee.ExitCode() // ran, but non-zero exit code
	}
	return err, -1
}

func equalDirs(t *testing.T, goldenDir, gotDir string) {
	t.Helper()
	getFiles := func(dir string) []fs.DirEntry {
		files, err := os.ReadDir(dir)
		if err != nil {
			t.Fatalf("%v", err)
		}
		return files
	}
	golden := getFiles(goldenDir)
	got := getFiles(gotDir)
	if len(golden) != len(got) {
		t.Fatalf("golden dir [%s] has %d files whereas got dir [%s] has %d", goldenDir, len(golden), gotDir, len(got))
	}
	for _, f := range golden {
		t.Run(f.Name(), func(t *testing.T) {
			goldenFile := filepath.Join(goldenDir, f.Name())
			gotFile := filepath.Join(gotDir, f.Name())
			diffs, err := DiffFiles(goldenFile, gotFile)
			if err != nil {
				t.Fatalf("%v", err)
			}
			if len(diffs) > 0 {
				
				t.Logf("differences between %s and %s\n %s", goldenFile, gotFile, string(diffs))
				t.Fail()
			}
		})
	}
}

// TestABC
// runs runabc() for each sample file and store the results in own temp dir
// and comapre each dir with the corresponding gold dir
func TestABC(t *testing.T) {
	dir := filepath.Join(testdir, "tmp")
	if update {
		dir = filepath.Join(testdir, "golden") //use the golden dir as destination
	} else {
		if err := os.RemoveAll(dir); err != nil { //delete test dir if it exists
			t.Fatalf("failed to remove dir %s for reasons other than not found", dir)
		}
	}
	for i := 1; i < 6; i++ {
		filename := "sample" + strconv.Itoa(i)
		//create  subdir for each sample output
		sampleDir := filepath.Join(dir, filename)
		if err := os.MkdirAll(sampleDir, 0755); err != nil {
			t.Fatalf("failed to create dir %s: %v ", sampleDir, err)
		}
		err, ec := runabc(sampleDir, filepath.Join(testdir, filename))
		if ec != 0 {
			t.Fatalf("processing %s failed with exit code= %d and err=%v", filename, ec, err)
		}
		if update {
			continue
		}
		goldenDir := filepath.Join(testdir, "golden", filename)
		equalDirs(t, goldenDir, sampleDir)
	}
}
