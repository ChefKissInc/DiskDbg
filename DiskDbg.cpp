#include <cctype>
#include <chrono>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
#include <thread>

namespace
{

    std::string runCommand(const std::string& cmd)
    {
        const auto pipe = popen(cmd.c_str(), "r");
        if (!pipe) {
            std::cerr << "Failed to run `" << cmd << "`\n";
            std::exit(EXIT_FAILURE);
        }

        char        buffer[1024];
        std::string output;

        while (fgets(buffer, sizeof(buffer), pipe)) { output += buffer; }

        if (const auto status = pclose(pipe); status != 0) {
            std::cerr << "Failed to run `" << cmd << "`\n";
            std::exit(EXIT_FAILURE);
        }
        return output;
    }

    void commandLoop(const std::string& prog, const std::string& args, const std::string& date,
                     const std::string& timestamp)
    {
        const auto cmd      = args.empty() ? prog : (prog + ' ' + args);
        const auto filename = "/Library/Logs/" + prog + "-" + timestamp + ".txt";
        const auto header =
            "Command: " + cmd + "\nDate: " + date + "\n#######################################################\n\n";

        size_t lastHash = 0;

        while (true) {
            std::string output = runCommand(cmd);

            if (const auto currentHash = std::hash<std::string>{}(output); currentHash != lastHash) {
                std::ofstream(filename) << header << output;
                lastHash = currentHash;
            }

            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
    }

    void rtrim(std::string& s)
    {
        s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) { return !std::isspace(ch); }).base(), s.end());
    }

}    // namespace

int main()
{
    auto date = runCommand("date +\"%Y-%m-%dT%H:%M:%S%z\"");
    rtrim(date);
    auto timestamp = runCommand("date +%s");
    rtrim(timestamp);

    std::thread t1(commandLoop, "dmesg", "", date, timestamp);
    std::thread t2(commandLoop, "ioreg", "-flxw0", date, timestamp);
    std::thread t3(commandLoop, "/System/Library/Extensions/AppleGraphicsControl.kext/Contents/MacOS/AGDCDiagnose", "",
                   date, timestamp);

    t1.join();
    t2.join();
    t3.join();
}
