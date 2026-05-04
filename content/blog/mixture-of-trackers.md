+++
title = "Mixture of Trackers (draft)"
date = 2025-09-02
draft = true

[taxonomies]
tags = ["VR", "Ideas"]
+++

## Draft Progress

- [x] First draft
- [ ] Second draft (cleanup & editing notes)
- [ ] Third draft (fix notes?)

## Draft

I've been playing a lot of VRChat recently, and one topic that keeps coming to mind is the immersion you can get from tracking the pose of your body. By default, for almost every virtual reality (VR) headset in existence, there are three trackers: your headset, and two controllers. That tells you very little about the player's pose; just the positions of the hands and the head. You can't really move your legs around, or your hips, or really anything at all. A lot of hardcore VRChat players invest in buying more trackers to fix this. Full Body Tracking (FBT) as it is known improves immersion significantly and makes it so much more fun to socialize! Some headsets have hand tracking, like the Meta Quest, and some even have eye and face tracking, the most exotic form of tracking in my opinion.

<!-- more -->

Having FBT, face tracking, eye tracking, and hand tracking setup seems like it would be incredible! Of course, the price for something like that is steep at best. You can try to set up a free solution that estimates your pose with a camera or an old Xbox Kinect, but the quality is shoddy at best. There's also a piece of software called "Standable: Full Body Estimation," which, as the name suggests, *estimates* the full body position based on the position of only the three tracker positions available in virtually all VR setups. You see, if you know those tracker positions, there are a plethora of techniques to estimate the rest of the body's pose based on what is plausible for the human body to achieve. Inverse Kinematics (IK) is well known and is used to estimate the position of the elbows, but from there you can estimate the shoulders, hips, and to an extent, the legs. Sure, you won't be able to kick or move them around, and the estimation cannot be fully accurate, but Standable stands as the best solution for pose estimation in VR that I have ever seen in any game or software.

Standable doesn't work with all the data it possibly could. Sure, it recently added support for "mixed tracking," or using a couple trackers from traditional FBT solutions to improve its estimation accuracy, but it doesn't utilize the camera feed from inside out tracking based headsets or webcam information that might be available. I think there's a bit of a missed opportunity here: what if you could utilize a *mixture* of pose estimation techniques that use a variety of heuristics and data sources to create an even more accurate estimation of your pose?

Consider the following situation: you have an inside out tracker based PCVR headset, a webcam on your PC, and Standable. Each source of data has strengths and weaknesses. Theoretically, the inside trackers could also be used to estimate the finger positions to some extent, as well as positions for the waist, hips, and legs/feet. The web cam can be used to predict a broader estimate on the full body pose. And Standable predicts a plausible pose based on the core three trackers (headset and controllers). Wouldn't it be interesting if you could *combine* these into a better pose than any method could predict on its own?

Let's take a small detour for a second. In the machine learning (ML) literature, there exists a technique known as Mixture of Experts (MoE). Essentially you have multiple models that can predict something, but they all specialize in something different. Some of their predictions are closer, some of them have worse predictions. The idea of MoE is that if you combine these expert predictions in some way, you can get a better prediction than any individual expert. [^1] Sounds familiar, doesn't it?

Lets apply this idea to our situation, specifically using focusing on feet tracking. This is certainly not Standable's forte, it has no way of predicting your feet position particularly well. The inside out trackers can do better though! However, the fail when your feet falls outside of their sight. Your PC's webcam helps in straighten out those situations.

Also consider the hip position, which I believe is a more interesting example. Standable is pretty good at getting an estimate of the hips based on where it thinks you have moved your center of mass, but it's not perfect. Sometimes, the inside out trackers can help inform the position occasionally, but I imagine most of the time it would get things completely wrong. Facing your webcam straight on, it would probably do well predicting the side to side motion of your hip, but perhaps it does worse predicting its motion towards and away from the camera (Standable would do better here!). Most of the time, each method's estimation is mediocre but could be improved, and perhaps if their predictions were combined, the overall result would be better than any individual expert, as in MoE!

Consider the fingers. Standable fails as it has nothing to work off of, but the inside out trackers can potentially provide hand tracking data using a method similar to what Meta Quest headsets use. In a Valve controller, we would also know the distances of fingers to the controller. To an extent the webcam could occasionally pick something up as well. While the inside out tracking provides a very diverse estimate, the Valve controller would provide accurate information on the finger's distance to the controller.

For one final example, consider that Standable supports improving its estimates through mixed tracking! Prior to creating the final estimate, an estimate based only on non-Standable estimates could be used with Standable's mixed tracking to create the final Standable estimate that is mixed with the other estimates. There could be other methods that support features that allow you to improve their tracking similar to Standable's mixed tracking, so this way of integrating these dependencies into my idea could be automated using dependency analysis.

This idea of using MoE in pose tracking is very promising in my opinion. It can help when one method fails by using the predictions of another method (perhaps by removing outliers in the case of nonsensical positions), as well as improving the results when various methods are close to the correct solution!

[^1]: These predictions are usually combined with a simple statistical model or through another "router" model that is trained to prioritize certain expert's predictions if that expert is a specialist for whatever the current task is.
