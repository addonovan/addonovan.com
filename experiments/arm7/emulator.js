
/* document set up */

var Emulator = {

  registerCount: 16,
  registers: [],

  address: {},
  labels: {},

  registerBase: 10,
  redraw: function( type ) {
    if ( type === undefined ) type = Emulator.registerBase;
    else Emulator.registerBase = type;

    for ( var i = 0; i < Emulator.registerCount; i++ )
    {
      var text = getRegisterObject( i );
      switch ( type )
      {
        case 2:
          text = text.toBinary();
          break;

        case 16:
          text = text.toHex();
          break;

        default:
          text = text.toDec();
          break;
      }
      $( registerSelector( i ) ).text( text );
    }
  },

  nextLine: function() {
    this.hideLine();
    Emulator.controls.offset += 4;
    this.updateLine();
  },

  hideLine: function() {
    $( "li[addr=" + Emulator.controls.offset + "]" ).removeClass( "selected" );
  },

  updateLine: function() {
    $( "li[addr=" + Emulator.controls.offset + "]" ).addClass( "selected" );
  },

  ready: function() {
    
  },

}

class Register
{
  constructor()
  {
    this.bits = [];
    for ( var i = 0; i < 32; i++ )
    {
      this.bits.push( 0 );
    }
  }

  set( num )
  {
    var it = toBits( num, 32 );
    for ( var i = 0; i < 32; i++ )
    {
      this.bits[ i ] = parseInt( it[ i ] );
    }
  }

  toBinary()
  {
    var string = "0b";
    for ( var i = 0; i < 32; i++ )
    {
      string += this.bits[ i ].toString();
    }
    return string;
  }

  toDec()
  {
    return parseInt( this.toBinary().substring( 3 ), 2 );
  }
  
  toHex()
  {
    return "0x" + this.toDec().toString( 16 ).toUpperCase();
  }
}

$( document ).ready( function() {
  $( "div#source" ).css( "margin-left", $( "div#tools" ).outerWidth() + 10 );
} );

$( "div#source ul li" ).first().each( function() {

  $( this ).addClass( "selected" );

} );

Emulator.ready = function()
{
  $( "pre#src" ).remove();
  var tools = $( "<div></div>" ).attr( "id", "tools" ).appendTo( "body" );

  // add controls
  $( "<div></div>" ).attr( "id", "controls" ).appendTo( tools );
  $( "div#controls" ).each( function() {

    var self = $( this );

    $( "<button></button>" ).html( "Run" ).click( function() {
      
      var self = $( this );
      if ( self.html() == "Run" )
      {
        self.html( "Stop" );
        Emulator.controls.run();
      }
      else
      {
        self.html( "Run" );
        Emulator.controls.stop();
      }

    } ).appendTo( self );
    self.append( "<br/>" );

    $( "<button></button>" ).html( "Step Once" ).attr( "onclick", "Emulator.controls.step()" ).appendTo( self );
    self.append( "<br/>" );

    $( "<button></button>" ).html( "Restart" ).attr( "onclick", "Emulator.controls.restart()" ).appendTo( self );
    self.append( "<br/>" );

    function radioButton( text, base ) {
      var label = $( "<label>" ).text( text );
      var input = $( "<input type='radio' name='registerBase'>" );

      input.click( function() {
        if ( $( this ).is( ":checked" ) )
        {
          Emulator.redraw( base );
        }
      } );

      self.append( input );
      self.append( label ).append( "<br/>" );
    }

    radioButton( "Hexadecimal", 16 );
    radioButton( "Decimal", 10 );
    radioButton( "Binary", 2 );

  } );

  // add registers
  $( "<div></div>" ).attr( "id", "registers" ).appendTo( tools );
  $( "div#registers" ).each( function() {

    var self = $(this);
    var table = $( "<table></table>" ).appendTo( self );

    for ( var i = 0; i < 16; i++ )
    {
      var name = "r" + i;
      
      var header = $( "<th></th>" ).html( name );
      var value = $( "<td></td>" ).html( "0" ).attr( "id", name );

      Emulator.registers[ i ] = new Register();

      $( "<tr></tr>" ).append( header ).append( value ).appendTo( table );
    }

    var test = "0b";
    for ( var i = 0; i < 32; i++ )
    {
      test += "0";
    }
    var row = $( "<tr></tr>" );
    row.append( "<td></td>" );
    $( "<td></td>" ).html( test ).css( "visibility", "hidden" ).appendTo( row );
    row.appendTo( table );
  } );

  // process the parsed source back into a displayable format
  var source = $( "<div></div>" ).attr( "id", "source" ).appendTo( "body" );
  var list = $( "<ul></ul>" ).appendTo( source );

}

/* emulation */

Emulator.restart = function() {
  for ( var i = 0; i < Emulator.registerCount; i++ )
  {
    Emulator.setRegister( i, 0 );
  }

  Emulator.hideLine();
  Emulator.currentLine = 0;
  Emulator.updateLine();
}

Emulator.getOp = function() {
  return $( "div#source li.selected span.op" ).html();
};

Emulator.getArg = function( i ) {
  return $( "div#source li.selected span.arg" ).eq( i ).html();
};

function registerSelector( i ) {
  var selector = "div#registers td#";
  if ( typeof( i ) === "string" )
  {
    selector += i;
  }
  else
  {
    selector += "r" + i;
  }
  return selector;
}

function getRegisterObject( i )
{
  if ( typeof( i ) === "string" ) 
  {
    return Emulator.registers[ parseInt( i.substring( 1 ) ) ];
  }
  else
  {
    return Emulator.registers[ i ];
  }
}

Emulator.setRegister = function( i, val ) {
  var obj = getRegisterObject( i );
  obj.set( val );
  $( registerSelector( i ) ).html( obj.toHex() );
};

Emulator.getRegister = function( i ) {
  return getRegisterObject( i ).toDec();
};

function isRegister( val ) {
  return val.toLowerCase().startsWith( "r" );
}

function fromLiteral( val ) {
  if ( typeof( val ) !== "string" ) return;
  if ( !val.startsWith( "#" ) ) return;

  // convert from binary
  if ( val.startsWith( "#0b" ) )
  {
    return parseInt( val.substring( 3 ), 2 );
  }
  // convert from hex
  else if ( val.startsWith( "#0x" ) )
  {
    return parseInt( val.substring( 3 ), 16 );
  }
  // convert from dec
  else
  {
    return parseInt( val.substring( 1 ) );
  }
}

function getValue( item )
{
  if ( isRegister( item ) )
  {
    return Emulator.getRegister( item );
  }
  else
  {
    return fromLiteral( item );
  }
}

function toBits( i, bits )
{
  var a = i.toString( 2 );
  while ( a.length < bits )
  {
    a = "0" + a;
  }
  return a;
}


