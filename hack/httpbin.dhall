-- Importing other files is done by specifying the HTTPS URL/disk location of
-- the file. Attaching a sha256 hash (obtained with `dhall freeze`) allows
-- the Dhall compiler to cache these files and speed up configuration loads
-- drastically.
let Prelude =
      https://raw.githubusercontent.com/dhall-lang/dhall-kubernetes/master/1.17/Prelude.dhall sha256:10db3c919c25e9046833df897a8ffe2701dc390fa0893d958c3430524be5a43e

let kubernetes =
      https://raw.githubusercontent.com/dhall-lang/dhall-kubernetes/master/1.17/package.dhall sha256:7150ac4309a091740321a3a3582e7695ee4b81732ce8f1ed1691c1c52791daa1

let deployment =
      kubernetes.Deployment::{
      , metadata = kubernetes.ObjectMeta::{
        , name = Some "httpbin"
        , namespace = Some "coretechexamples"
        }
      , spec = Some kubernetes.DeploymentSpec::{
        , selector = kubernetes.LabelSelector::{
          , matchLabels = Some (toMap { app = "httpbin" })
          }
        , template = kubernetes.PodTemplateSpec::{
          , metadata = kubernetes.ObjectMeta::{
            , name = Some "httpbin"
            , labels = Some (toMap { app = "httpbin" })
            }
          , spec = Some kubernetes.PodSpec::{
            , serviceAccountName = Some "httpbin"
            , containers =
              [ kubernetes.Container::{
                , name = "httpbin"
                , image = Some "kennethreitz/httpbin"
                , imagePullPolicy = Some "Always"
                , command = Some
                  [ "gunicorn"
                  , "--access-logfile"
                  , "-"
                  , "-b"
                  , "0.0.0.0:8080"
                  , "httpbin:app"
                  ]
                , ports = Some
                  [ kubernetes.ContainerPort::{
                    , containerPort = 8080
                    , name = Some "http"
                    }
                  ]
                , readinessProbe = Some kubernetes.Probe::{
                  , failureThreshold = Some 3
                  , httpGet = Some kubernetes.HTTPGetAction::{
                    , path = Some "/status/200"
                    , port = kubernetes.IntOrString.Int 8080
                    , scheme = Some "HTTP"
                    }
                  , periodSeconds = Some 10
                  , successThreshold = Some 1
                  , timeoutSeconds = Some 1
                  }
                , resources = Some
                  { limits = Some (toMap { cpu = "500m" })
                  , requests = Some (toMap { cpu = "10m" })
                  }
                }
              ]
            , securityContext = Some kubernetes.PodSecurityContext::{
              , runAsUser = Some 1000
              , runAsGroup = Some 1000
              , fsGroup = Some 1000
              }
            }
          }
        }
      }

in  deployment
