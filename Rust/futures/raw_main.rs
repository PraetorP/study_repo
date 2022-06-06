#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod executor {


    // Spawn a task to print before and after waiting on a timer.
    // Wait for our timer future to complete after two seconds.

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    use ::{};
    use futures::{};
    use futures::future::{};
    use futures::future::BoxFuture;
    use futures::future::FutureExt;
    use futures::task::{};
    use futures::task::waker_ref;
    use futures::task::ArcWake;
    use std::{};
    use std::future::Future;
    use std::sync::mpsc::{};
    use std::sync::mpsc::sync_channel;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::SyncSender;
    use std::sync::{};
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::task::{};
    use std::task::Context;
    use std::task::Poll;
    use std::time::Duration;
    /// Task executor that receives tasks off of a channel and runs them.
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }
    impl Executor {
        fn run(self:
                &Self) {
                loop {
                        if let Ok(task) = self.ready_queue.recv()
                                {
                                        let mut future_slot = task.future.lock().unwrap();
                                        if let Some(mut future) = future_slot.take()
                                                {
                                                        let waker = waker_ref(&task);
                                                        let context = &mut Context::from_waker(&*waker);
                                                        if future.as_mut().poll(context).is_pending()
                                                                { *future_slot = Some(future); }
                                                            } } else { break; } } }
                            }
                            /// `Spawner` spawns new futures onto the task channel.
                            struct Spawner {
                                task_sender: SyncSender<Arc<Task>>,
                            }
                            #[automatically_derived]
                            #[allow(unused_qualifications)]
                            impl ::core::clone::Clone for Spawner {
                                #[inline]
                                fn clone(self: &Self)
                                    ->
                                        Spawner {
                                        match *self {
                                                Spawner { task_sender: ref __self_0_0 } =>
                                                    Spawner{task_sender:
                                                            ::core::clone::Clone::clone(&*__self_0_0),},
                                            }
                                    }
                            }
                            impl Spawner {
                                fn spawn<impl Future<Output = ()> + 'static + Send>(self:
                                        &Self, future: impl Future<Output = ()> + 'static + Send)
                                    where
                                    impl Future<Output = ()> + 'static + Send: Future<Output =
                                    ()> + 'static +
                                    Send {
                                        let future = future.boxed();
                                        let task =
                                            Arc::new(Task{future: Mutex::new(Some(future)),
                                                    task_sender: self.task_sender.clone(),});
                                        self.task_sender.send(task).expect("too many tasks queued");
                                    }
                            }
                            /// A future that can reschedule itself to be polled by an `Executor`.
                            struct Task {
                                /// In-progress future that should be pushed to completion.
                                ///
                                /// The `Mutex` is not necessary for correctness, since we only have
                                /// one thread executing tasks at once. However, Rust isn't smart
                                /// enough to know that `future` is only mutated from one thread,
                                /// so we need to use the `Mutex` to prove thread-safety. A production
                                /// executor would not need this, and could use `UnsafeCell` instead.
                                future: Mutex<Option<BoxFuture>>,
                                /// Handle to place the task itself back onto the task queue.
                                task_sender: SyncSender<Arc<Task>>,
                            }
                            impl ArcWake for Task {
                                fn wake_by_ref(arc_self:
                                        &Arc<Self>) {
                                        let cloned = arc_self.clone();
                                        arc_self.task_sender.send(cloned).expect("too many tasks queued");
                                    }
                            }
                            fn new_executor_and_spawner()
                                ->
                                    (Executor,
                                    Spawner) {
                                    const MAX_QUEUED_TASKS: usize = 10000;
                                    let (task_sender, ready_queue) =
                                        sync_channel(MAX_QUEUED_TASKS);
                                    (Executor{ready_queue,}, Spawner{task_sender,})
                                } }
                        mod time_future {
                            use std::{};
                            use std::future::Future;
                            use std::pin::Pin;
                            use std::sync::{};
                            use std::sync::Arc;
                            use std::sync::Mutex;
                            use std::task::{};
                            use std::task::Context;
                            use std::task::Poll;
                            use std::task::Waker;
                            use std::thread;
                            use std::time::Duration;
                            struct TimerFuture {
                                shared_state: Arc<Mutex<SharedState>>,
                            }
                            /// Shared state between the future and the waiting thread
                            struct SharedState {
                                /// Whether or not the sleep time has elapsed
                                completed: bool,
                                /// The waker for the task that `TimerFuture` is running on.
                                /// The thread can use this after setting `completed = true` to tell
                                /// `TimerFuture`'s task to wake up, see that `completed = true`, and
                                /// move forward.
                                waker: Option<Waker>,
                            }
                            impl Future for TimerFuture {
                                type
                                Output
                                =
                                ();
                                fn poll(self: Pin<&mut Self>, cx: &mut Context)
                                    ->
                                        Poll<Self::Output> {
                                        let mut shared_state = self.shared_state.lock().unwrap();
                                        if shared_state.completed
                                                {
                                                        Poll::Ready(())
                                                    } else {
                                                   shared_state.waker = Some(cx.waker().clone());
                                                   Poll::Pending
                                               }
                                            }
                                    }
                                    impl TimerFuture {
                                        /// Create a new `TimerFuture` which will complete after the provided
                                        /// timeout.
                                        fn new(duration: Duration)
                                            ->
                                                Self {
                                                let shared_state =
                                                    Arc::new(Mutex::new(SharedState{completed: false,
                                                                waker: None,}));
                                                let thread_shared_state = shared_state.clone();
                                                thread::spawn(move ||
                                                        {
                                                                thread::sleep(duration);
                                                                let mut shared_state = thread_shared_state.lock().unwrap();
                                                                shared_state.completed = true;
                                                                if let Some(waker) = shared_state.waker.take()
                                                                        { waker.wake() }
                                                                    }); TimerFuture{shared_state,} }
                                            } }
                                        use std::time::Duration;
                                        use executor::new_executor_and_spawner;
                                        use time_future::TimerFuture;
                                        fn main() {
                                                let (executor, spawner) = new_executor_and_spawner();
                                                spawner.spawn(#[lang = "from_generator"](|mut _task_context|
                                                            {
                                                                    {
                                                                            ::std::io::_print(::core::fmt::Arguments::new_v1(&["howdy!\n"],
                                                                                    &[]));
                                                                        };
                                                                    match #[lang = "into_future"](TimerFuture::new(Duration::new(2,
                                                                                            0))) {
                                                                            mut __awaitee =>
                                                                                loop {
                                                                                        match unsafe {
                                                                                                            #[lang = "poll"](#[lang = "new_unchecked"](&mut __awaitee),
                                                                                                                #[lang = "get_context"](_task_context))
                                                                                                        } {
                                                                                                #[lang = "Ready"] { 0: result } => break result,
                                                                                                #[lang = "Pending"] {} => { }
                                                                                            }
                                                                                        _task_context = (yield ());
                                                                                    },
                                                                        };
                                                                    {
                                                                            ::std::io::_print(::core::fmt::Arguments::new_v1(&["done!\n"],
                                                                                    &[]));
                                                                        };
                                                                }));
                                                drop(spawner);
                                                executor.run();
                                            }
