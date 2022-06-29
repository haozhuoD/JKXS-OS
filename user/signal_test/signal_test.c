#include <stdio.h>
#include <signal.h>
#include <unistd.h>

int pid = 0;
int cnt1 = 0;
int cnt2 = 0;

void func_1(int _x)
{
    cnt1 += 1;
    int x = cnt1;
    if (x > 3) {
        // cout << "A " << x << " =" << endl;
        return;
    }
    // sigset_t set;
    // sigemptyset(&set);
    // sigaddset(&set, SIGINT);
    // sigprocmask(SIG_UNBLOCK, &set, NULL);
    printf("A %d +\n", x);
    kill(pid, SIGINT);
    kill(pid, SIGCONT);
    printf("A %d -\n", x);
}

void func_2(int _x)
{
    cnt2 += 1;
    int x = cnt2;
    if (x > 3) {
        // cout << "B " << x << " =" << endl;
        return;
    }
    printf("B %d +\n", x);
    kill(pid, SIGINT);
    kill(pid, SIGCONT);
    printf("B %d -\n", x);
}

int main()
{
    signal(SIGINT, func_1);
    signal(SIGCONT, func_2);
    pid = getpid();
    printf("pid: %d\n", pid);
    kill(pid, SIGINT);
    kill(pid, SIGCONT);
    return 0;
}