/*
 * devolutions-gateway
 *
 * Protocol-aware fine-grained relay server
 *
 * The version of the OpenAPI document: 2025.1.4
 * Contact: infos@devolutions.net
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using System.Text.RegularExpressions;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using System.ComponentModel.DataAnnotations;
using FileParameter = Devolutions.Gateway.Client.Client.FileParameter;
using OpenAPIDateConverter = Devolutions.Gateway.Client.Client.OpenAPIDateConverter;

namespace Devolutions.Gateway.Client.Model
{
    /// <summary>
    /// Defines CredentialsKind
    /// </summary>
    [JsonConverter(typeof(StringEnumConverter))]
    public enum CredentialsKind
    {
        /// <summary>
        /// Enum UsernamePassword for value: username-password
        /// </summary>
        [EnumMember(Value = "username-password")]
        UsernamePassword = 1
    }

    public static class CredentialsKindExtensions
    {
        /// <summary>
        /// Returns the value as string for a given variant
        /// </summary>
        public static string ToValue(this CredentialsKind variant)
        {
            switch (variant)
            {
                case CredentialsKind.UsernamePassword:
                    return "username-password";
                default:
                    throw new ArgumentOutOfRangeException(nameof(variant), $"Unexpected variant: {variant}");
            }
        }
    }

}
