using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;

using UnityEngine;
using UnityEngine.UI;
using UnityEngine.Playables;
using UnityEngine.Networking;
using System.Runtime.InteropServices;


public class Plugin
{
    [DllImport("plugin.so")]
    public static extern int hello();

    [DllImport("plugin.so")]
    public static extern int spawn_tokio_runtime();

    [DllImport("plugin.so")]
    public static extern int test();
}

public class GrpcTest : MonoBehaviour
{
    void Start()
    {
        Environment.SetEnvironmentVariable("RUST_LOG", "plugin");
        Environment.SetEnvironmentVariable("RUST_BACKTRACE", "1");
    }

    void Update()
    {

    }

    static Barrier runtimeStarted = new Barrier(participantCount: 1);

    public string testAddress = "127.0.0.1";
    public int testPort = 50051;
    public bool useTls = false;
    public bool sendAuthToken = false;
    public bool readStreamBeforeDispose = false;

    private static readonly DateTime UnixEpoch = new DateTime(1970, 1, 1, 0, 0, 0);
    private ulong GetEpochTimestampNS()
    {
        var timeSpan = (DateTime.UtcNow - UnixEpoch);
        ulong seconds = (ulong)timeSpan.TotalSeconds;
        uint nanos = (uint)((timeSpan.TotalSeconds - seconds) * 1e9);
        ulong timestampNS = seconds * (ulong)1000000000 + nanos;
        return timestampNS;
    }

    private static void TokioRuntime()
    {
        Debug.Log("TokioRuntime thread");
        Plugin.spawn_tokio_runtime();
    }

    public async void RunGrpcTest()
    {
        Debug.Log("test = " + Plugin.hello());

        Debug.Log("Spawning tokio runtime...");
        var rt = new Thread(TokioRuntime);
        rt.Start();

        await Task.Delay(5000);

        Debug.Log("grpc test...");
        Plugin.test();
    }
}
