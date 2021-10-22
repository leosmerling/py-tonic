# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: hw.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='hw.proto',
  package='hw',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\x08hw.proto\x12\x02hw\"\"\n\x05\x45xtra\x12\x0c\n\x04text\x18\x01 \x01(\t\x12\x0b\n\x03num\x18\x02 \x01(\x04\"6\n\x0cHelloRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x18\n\x05\x65xtra\x18\x02 \x01(\x0b\x32\t.hw.Extra\":\n\rHelloResponse\x12\x0f\n\x07message\x18\x01 \x01(\t\x12\x18\n\x05\x65xtra\x18\x02 \x01(\x0b\x32\t.hw.Extra2:\n\x07Greeter\x12/\n\x08SayHello\x12\x10.hw.HelloRequest\x1a\x11.hw.HelloResponseb\x06proto3'
)




_EXTRA = _descriptor.Descriptor(
  name='Extra',
  full_name='hw.Extra',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='text', full_name='hw.Extra.text', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='num', full_name='hw.Extra.num', index=1,
      number=2, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=16,
  serialized_end=50,
)


_HELLOREQUEST = _descriptor.Descriptor(
  name='HelloRequest',
  full_name='hw.HelloRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='hw.HelloRequest.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='extra', full_name='hw.HelloRequest.extra', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=52,
  serialized_end=106,
)


_HELLORESPONSE = _descriptor.Descriptor(
  name='HelloResponse',
  full_name='hw.HelloResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='message', full_name='hw.HelloResponse.message', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='extra', full_name='hw.HelloResponse.extra', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=108,
  serialized_end=166,
)

_HELLOREQUEST.fields_by_name['extra'].message_type = _EXTRA
_HELLORESPONSE.fields_by_name['extra'].message_type = _EXTRA
DESCRIPTOR.message_types_by_name['Extra'] = _EXTRA
DESCRIPTOR.message_types_by_name['HelloRequest'] = _HELLOREQUEST
DESCRIPTOR.message_types_by_name['HelloResponse'] = _HELLORESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Extra = _reflection.GeneratedProtocolMessageType('Extra', (_message.Message,), {
  'DESCRIPTOR' : _EXTRA,
  '__module__' : 'hw_pb2'
  # @@protoc_insertion_point(class_scope:hw.Extra)
  })
_sym_db.RegisterMessage(Extra)

HelloRequest = _reflection.GeneratedProtocolMessageType('HelloRequest', (_message.Message,), {
  'DESCRIPTOR' : _HELLOREQUEST,
  '__module__' : 'hw_pb2'
  # @@protoc_insertion_point(class_scope:hw.HelloRequest)
  })
_sym_db.RegisterMessage(HelloRequest)

HelloResponse = _reflection.GeneratedProtocolMessageType('HelloResponse', (_message.Message,), {
  'DESCRIPTOR' : _HELLORESPONSE,
  '__module__' : 'hw_pb2'
  # @@protoc_insertion_point(class_scope:hw.HelloResponse)
  })
_sym_db.RegisterMessage(HelloResponse)



_GREETER = _descriptor.ServiceDescriptor(
  name='Greeter',
  full_name='hw.Greeter',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_start=168,
  serialized_end=226,
  methods=[
  _descriptor.MethodDescriptor(
    name='SayHello',
    full_name='hw.Greeter.SayHello',
    index=0,
    containing_service=None,
    input_type=_HELLOREQUEST,
    output_type=_HELLORESPONSE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
])
_sym_db.RegisterServiceDescriptor(_GREETER)

DESCRIPTOR.services_by_name['Greeter'] = _GREETER

# @@protoc_insertion_point(module_scope)